#!/usr/bin/env python3
"""Posts Rosetta news to Discord webhooks. CI runs it after a push to main:

  announce.py scores BEFORE AFTER   official results that are new or changed between two commits
                                    (site/results/index.json), for the score-updates channel
  announce.py suite BEFORE AFTER    a new suite name or changed hand-written cases, for the
                                    benchmark-updates channel
  announce.py message TEXT          a message written by hand (the workflow's manual run)

The webhook URL comes from DISCORD_WEBHOOK. Without one (a fork, or a local run) the payload is
printed instead of sent, which is also what --dry-run does."""

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.request

SITE = "https://scrensayol.github.io/rosetta/"
REPO = "https://github.com/Scrensayol/rosetta"
ACCENT = 0xFF8AB4
ZERO = "0" * 40


def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True)


def show(commit, path):
    """A file's text at a commit, or None when it isn't there."""
    r = git("show", f"{commit}:{path}")
    return r.stdout if r.returncode == 0 else None


def usable(commit):
    return commit and commit != ZERO and git("cat-file", "-e", f"{commit}^{{commit}}").returncode == 0


def failures(totals):
    # what the leaderboard's "wrong" column counts
    return totals.get("mismatch", 0) + totals.get("recompile_failed", 0) + totals.get("decompile_failed", 0)


def pct(score):
    return score["passed"] / score["total"] * 100 if score["total"] else 0.0


def fmt_pct(v):
    # never round a miss up to 100%
    if v == 100:
        return "100%"
    return f"{min(v, 99.99):.2f}%"


def page(entry):
    return f"{SITE}result.html?r={entry['file'].removesuffix('.json')}"


def scores(before, after):
    old = json.loads(show(before, "site/results/index.json") or '{"results": [], "suites": {}}')
    new = json.loads(show(after, "site/results/index.json") or '{"results": [], "suites": {}}')
    was = {r["file"]: r for r in old["results"]}
    # results moved into an archive directory when the suite changed aren't news
    changed = [r for r in new["results"] if not r.get("archived") and was.get(r["file"], {}).get("sha256") != r["sha256"]]
    if not changed:
        print("no official result changed")
        return []

    def suite_name(index, entry):
        return index["suites"].get(entry["hash"], {}).get("name", "?")

    # one embed per suite and tier, ranked the way the leaderboard ranks
    groups = {}
    for r in changed:
        groups.setdefault((suite_name(new, r), r["tier"], r["hash"]), []).append(r)
    embeds = []
    for (suite, tier, hash), rows in sorted(groups.items(), key=lambda g: (g[0][0], g[0][1] != "quick")):
        board = sorted(
            (r for r in new["results"] if r["hash"] == hash and not r.get("archived")),
            key=lambda r: (-pct(r["score"]), failures(r["totals"])),
        )
        lines = []
        for r in sorted(rows, key=board.index):
            rank = board.index(r) + 1
            who = f"**{r['decompiler']}**" + (f" {r['version']}" if r.get("version") else "")
            s = r["score"]
            line = (
                f"#{rank} {who}: **{s['passed']:,} / {s['total']:,}** ({fmt_pct(pct(s))})"
                f" · {failures(r['totals']):,} wrong · {r['totals'].get('refused', 0):,} refused · [result]({page(r)})"
            )
            prev = was.get(r["file"])
            if prev and prev["hash"] == r["hash"]:
                delta = r["score"]["passed"] - prev["score"]["passed"]
                pv = f" {prev['version']}" if prev.get("version") and prev.get("version") != r.get("version") else ""
                line += f"\n-# was {prev['score']['passed']:,}{pv} ({'+' if delta >= 0 else ''}{delta:,})"
            elif prev:
                line += f"\n-# first run on {suite}"
            lines.append(line)
        embeds.append({
            "title": f"{suite} · {tier} tier",
            "url": SITE + "#leaderboard",
            "color": ACCENT,
            "description": "\n".join(lines)[:4000],
            "footer": {"text": f"{len(board)} on the board · {after[:7]}"},
        })
    title = "new official result" if len(changed) == 1 else f"{len(changed)} new official results"
    return [{"content": f"**{title}**", "embeds": embeds[i:i + 10]} for i in range(0, len(embeds), 10)]


SUITE_RE = re.compile(r'pub const SUITE: &str = "([^"]+)";')


def about(text):
    """A case's opening comment block as one line, like bench.rs's about()."""
    lines = []
    for line in text.splitlines():
        if not lines and line.startswith("--!"):
            continue
        if not line.startswith("--"):
            break
        lines.append(line[2:].strip())
    text = " ".join(l for l in lines if l)
    return text if len(text) <= 300 else text[:297] + "..."


def suite(before, after):
    old_name = SUITE_RE.search(show(before, "src/bench.rs") or "")
    new_name = SUITE_RE.search(show(after, "src/bench.rs") or "")
    old_name = old_name and old_name.group(1)
    new_name = new_name and new_name.group(1)
    diff = git("diff", "--name-status", "--no-renames", before, after, "--", "cases/").stdout.split("\n")
    added, removed, modified = [], [], []
    for row in filter(None, diff):
        status, path = row.split("\t", 1)
        name = os.path.splitext(os.path.basename(path))[0]
        if status == "A":
            added.append((name, about(show(after, path) or "")))
        elif status == "D":
            removed.append(name)
        else:
            modified.append(name)
    if old_name == new_name and not (added or removed or modified):
        print("the suite didn't change")
        return []

    fields = []
    if added:
        lines = []
        for i, (n, a) in enumerate(added):
            line = f"**{n}**: {a[:157] + '...' if len(a) > 160 else a}" if a else f"**{n}**"
            more = f"\n…and {len(added) - i} more"
            if len("\n".join(lines + [line])) + len(more) > 1024:
                lines.append(more.strip())
                break
            lines.append(line)
        fields.append({"name": f"new cases ({len(added)})", "value": "\n".join(lines)})
    if modified:
        fields.append({"name": "changed cases", "value": ", ".join(modified)[:1024]})
    if removed:
        fields.append({"name": "removed cases", "value": ", ".join(removed)[:1024]})
    log = git("log", "--format=%s", f"{before}..{after}", "--", "cases/", "src/").stdout.strip()
    if log:
        fields.append({"name": "changes", "value": "\n".join(f"- {l}" for l in log.splitlines()[:12])[:1024]})

    if old_name != new_name:
        title = f"suite {new_name} is out"
        description = (
            f"it replaces {old_name}. scores on different suites ran different programs, so they don't compare: "
            f"official reruns on {new_name} go to the score updates channel as they land."
        )
    else:
        title = f"suite {new_name}: the cases changed"
        description = "the hand-written cases changed without a new suite name."
    return [{
        "content": f"**{title}**",
        "embeds": [{
            "title": title,
            "url": f"{REPO}/compare/{before[:12]}...{after[:12]}",
            "color": ACCENT,
            "description": description,
            "fields": fields,
            "footer": {"text": after[:7]},
        }],
    }]


def send(payloads, dry):
    url = os.environ.get("DISCORD_WEBHOOK", "").strip()
    for p in payloads:
        # names and versions come from result files: never let them ping anyone
        p["allowed_mentions"] = {"parse": []}
        if dry or not url:
            print(json.dumps(p, indent=2, ensure_ascii=False))
            continue
        req = urllib.request.Request(
            url,
            data=json.dumps(p).encode(),
            headers={"Content-Type": "application/json", "User-Agent": "rosetta-announce (github actions)"},
        )
        try:
            urllib.request.urlopen(req, timeout=20).read()
        except urllib.error.HTTPError as e:
            sys.exit(f"discord said {e.code}: {e.read().decode(errors='replace')[:300]}")
    if payloads and not dry and not url:
        print("DISCORD_WEBHOOK isn't set: printed instead of sent")


def main():
    args = [a for a in sys.argv[1:] if a != "--dry-run"]
    dry = len(args) != len(sys.argv) - 1
    if len(args) == 2 and args[0] == "message":
        send([{"content": args[1][:2000]}], dry)
        return
    if len(args) != 3 or args[0] not in ("scores", "suite"):
        sys.exit(__doc__)
    kind, before, after = args
    if not usable(before) or not usable(after):
        print(f"no usable range {before}..{after} (a new branch or a rewritten history): nothing to compare")
        return
    before, after = (git("rev-parse", c).stdout.strip() for c in (before, after))
    send((scores if kind == "scores" else suite)(before, after), dry)


if __name__ == "__main__":
    main()

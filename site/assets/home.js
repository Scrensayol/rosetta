// the landing page
"use strict";

nav("luau decompiler benchmark");
constellation(document.getElementById("sky"));

document.getElementById("copy").addEventListener("click", async e => {
	const text = document.querySelector(".code pre").textContent.split("\n").filter(l => !l.startsWith("#")).join("\n").trim();
	await navigator.clipboard.writeText(text);
	const label = e.currentTarget.querySelector("span");
	label.textContent = "copied";
	setTimeout(() => (label.textContent = "copy"), 1400);
});

const FEATURES = [
	["flask", "round-trip grading", "the reference is the original bytecode's behavior, never the source text. the output has to recompile at the same profile and print the same thing."],
	["shield", "no self-grading", "every decompiler gets the same bytes and the same judge. rosetta doesn't know or care whose output it's running."],
	["shuffle", "programs nobody tuned for", "hand-written edge cases, programs generated from a seed, and mutants kept only when their bytecode shows something new."],
	["layers", "every optimization level", "each program runs at <span class=keep>-O0</span>, <span class=keep>-O1</span> and <span class=keep>-O2</span>, with debug level 1 and 2. inlining and constant folding are where decompilers break."],
	["ban", "refusing isn't lying", "a clean error exit is a refusal, counted apart from wrong output. a decompiler that knows its limits shouldn't score like one that guesses."],
	["eye", "readability, measured", "passing output is scored against the source: token similarity, size, locals, nesting, added endless loops and recovered names. no llm judge."],
];
document.getElementById("features").innerHTML = FEATURES.map(([ic, title, text], i) =>
	`<div class="feature rise${i === 5 ? "" : ""}" style="--i:${i}"><h3>${icon(ic)}${title}</h3><p>${text}</p></div>`).join("");

// ---- leaderboard ----
// results from different suites ran different programs, so the board shows one suite at a time:
// the newest by default, older ones (archived under results/<suite>/) a click away
const suiteNumber = name => Number((name || "").match(/(\d+)$/)?.[1] || 0);
let suite = null;
// ?tier=hard-quick opens the board on that tier
// (plain "hard" from before the hard tier had two sizes opens hard-quick)
let tier = ({ hard: "hard-quick" })[new URLSearchParams(location.search).get("tier")] || new URLSearchParams(location.search).get("tier") || "quick";
const TIER_ORDER = ["quick", "full", "hard-quick", "hard-full"];
async function board() {
	const index = await officialResults();
	const suiteOf = r => index.suites[r.hash]?.name || "?";
	const suites = [...new Set(index.results.map(suiteOf))].sort((a, b) => suiteNumber(b) - suiteNumber(a) || a.localeCompare(b));
	if (!suites.includes(suite)) suite = suites[0] || "rosetta";
	const inSuite = index.results.filter(r => suiteOf(r) === suite);
	const rank = t => (TIER_ORDER.includes(t) ? TIER_ORDER.indexOf(t) : TIER_ORDER.length);
	const tiers = [...new Set(inSuite.map(r => r.tier))].sort((a, b) => rank(a) - rank(b) || a.localeCompare(b));
	if (!tiers.includes(tier)) tier = tiers[0] || "quick";
	const rows = inSuite
		.filter(r => r.tier === tier)
		.sort((a, b) => pct(b.score) - pct(a.score) || failures(a.totals) - failures(b.totals));
	const el = document.getElementById("board");
	const info = rows[0] ? index.suites[rows[0].hash] : null;
	const old = suite !== suites[0];
	el.innerHTML = `
		<div class="board-head"><h3>suite <span class="keep">${esc(suite)}</span>${old ? ' <small class="dim">archived</small>' : ""}</h3>
			<div class="chips">${suites.length > 1 ? suites.map(s => `<button class="chip ${s === suite ? "on" : ""}" data-suite="${esc(s)}">${esc(s)}</button>`).join("") + '<span class="chip-gap"></span>' : ""}${tiers.map(t => `<button class="chip ${t === tier ? "on" : ""}" data-tier="${t}">${t}</button>`).join("")}</div></div>
		${tier.startsWith("hard") ? x3(rows) : ""}
		<table>
			<thead><tr><th class="rank">#</th><th>decompiler</th><th class="meter">round trips passed</th>
				<th class="num">wrong</th><th class="num hide-sm">refused</th><th class="num hide-sm">time</th></tr></thead>
			<tbody>${rows.map((r, i) => {
				const wrong = failures(r.totals), refused = r.totals.refused || 0;
				// a tie shares its rank
				const same = (a, b) => a && a.score.passed === b.score.passed && a.score.total === b.score.total && failures(a.totals) === failures(b.totals);
				let rank = i;
				while (rank > 0 && same(rows[rank - 1], r)) rank--;
				return `<tr class="rise" style="--i:${i}" data-file="${esc(r.file.replace(/\.json$/, ""))}">
					<td class="rank">${rank + 1}</td>
					<td class="who">${esc(r.decompiler)}<small class="keep">${esc(r.version || "")}</small></td>
					<td class="meter"><div class="meter-row"><div class="track2"><i data-w="${pct(r.score)}%"></i></div><b>${fmtPct(pct(r.score))}</b></div></td>
					<td class="num ${wrong ? "bad" : "dim"}">${wrong.toLocaleString()}</td>
					<td class="num hide-sm ${refused ? "warn" : "dim"}">${refused.toLocaleString()}</td>
					<td class="num hide-sm dim">${r.web ? "web api" : `${Number(r.seconds).toFixed(1)}s`}</td></tr>`;
			}).join("") || '<tr><td colspan="6" class="dim" style="padding:18px 14px">no official results yet.</td></tr>'}</tbody>
		</table>
		<div class="board-foot">${info ? `${Object.entries(info.sections).map(([k, v]) => `${v.count} ${k}`).join(" · ")} · each at ${info.profiles} profiles. ` : ""}${old ? "an older suite: its scores don't compare with the current one's. " : ""}${tier.startsWith("hard") ? "the hard tiers rewrite each program's bytecode before the decompiler sees it (same behavior, not the compiler's shapes): robustness, scored apart from quick and full. " : ""}time is the wall clock for the whole run on one machine; web apis are rate-limited, so theirs measures the network.</div>`;
	document.querySelectorAll(".suite-name").forEach(s => (s.textContent = suites[0] || s.textContent));
	el.querySelectorAll("[data-suite]").forEach(b => b.addEventListener("click", () => { suite = b.dataset.suite; board(); }));
	el.querySelectorAll("[data-tier]").forEach(b => b.addEventListener("click", () => { tier = b.dataset.tier; board(); }));
	el.querySelectorAll("tr[data-file]").forEach(tr => tr.addEventListener("click", () => (location.href = `result?r=${encodeURIComponent(tr.dataset.file)}`)));
	animateIn(el);
}
board();

// the hard tier's top level: cleared means every one of its round trips passed
function x3(rows) {
	const at3 = rows.filter(r => r.levels?.["3"]?.total);
	const cleared = at3.filter(r => r.levels["3"].passed === r.levels["3"].total);
	const best = at3.slice().sort((a, b) => b.levels["3"].passed - a.levels["3"].passed)[0];
	if (cleared.length) {
		return `<div class="board-callout cleared">${icon("check")}<span><b>x3 cleared</b> by ${cleared.map(r => esc(r.decompiler)).join(", ")}</span></div>`;
	}
	return `<div class="board-callout">${icon("warn")}<span><b>x3:</b> everyone just gets fucking killed here${best && best.levels["3"].passed ? ` <small>best so far: ${esc(best.decompiler)}, ${best.levels["3"].passed} / ${best.levels["3"].total}</small>` : ""}</span></div>`;
}

// ---- the three kinds of result ----
const STATES = [
	["spoofed", "warn", "var(--state-error)", "spoofed result", "the file's own numbers don't add up, or it claims the suite but ran other programs. the viewer says so up top.", 0.9, "81", "90", "9 wrong"],
	["unverified", "bang", "var(--state-warn)", "unverified result", "it adds up, but rosetta didn't publish it. fine for testing your own decompiler; just not proof.", 0.92, "83", "90", "7 wrong"],
	["verified", "check", "var(--accent)", "verified result", "byte for byte an official run from the rosetta repo, listed in its index with a sha-256.", 0.87, "79", "90", "11 wrong"],
];
document.getElementById("states").innerHTML = STATES.map(([cls, ic, color, title, text, f, a, b, sub], i) => `
	<div class="state ${cls} rise" style="--i:${i * 2}">
		<h3>${icon(ic, `style="fill:${color}"`)}${title}</h3>
		<p>${text}</p>
		<div class="mini">${ring(f, Math.round(f * 100) + "%")}<div><div class="mini-score">${a}<span> / ${b}</span></div><small>${sub}</small></div></div>
	</div>`).join("");
new IntersectionObserver((entries, obs) => {
	for (const e of entries) if (e.isIntersecting) { animateIn(e.target); obs.unobserve(e.target); }
}, { threshold: 0.3 }).observe(document.getElementById("states"));

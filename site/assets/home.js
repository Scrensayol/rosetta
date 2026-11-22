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
let tier = "quick";
async function board() {
	const index = await officialResults();
	const tiers = [...new Set(index.results.map(r => r.tier))].sort((a, b) => (a === "quick" ? -1 : b === "quick" ? 1 : 0));
	if (!tiers.includes(tier)) tier = tiers[0] || "quick";
	const rows = index.results
		.filter(r => r.tier === tier)
		.sort((a, b) => pct(b.score) - pct(a.score) || failures(a.totals) - failures(b.totals));
	const el = document.getElementById("board");
	const suite = rows[0] ? index.suites[rows[0].hash] : null;
	el.innerHTML = `
		<div class="board-head"><h3>suite <span class="keep">rosetta-1</span></h3>
			<div class="chips">${tiers.map(t => `<button class="chip ${t === tier ? "on" : ""}" data-tier="${t}">${t}</button>`).join("")}</div></div>
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
		<div class="board-foot">${suite ? `${Object.entries(suite.sections).map(([k, v]) => `${v.count} ${k}`).join(" · ")} · each at ${suite.profiles} profiles. ` : ""}time is the wall clock for the whole run on one machine; web apis are rate-limited, so theirs measures the network.</div>`;
	el.querySelectorAll("[data-tier]").forEach(b => b.addEventListener("click", () => { tier = b.dataset.tier; board(); }));
	el.querySelectorAll("tr[data-file]").forEach(tr => tr.addEventListener("click", () => (location.href = `result.html?r=${encodeURIComponent(tr.dataset.file)}`)));
	animateIn(el);
}
board();

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

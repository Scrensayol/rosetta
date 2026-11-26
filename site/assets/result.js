// the result page: a file from results/, a link, one shared through Rubiš, or one dropped in
const RUBIS = "https://api.rubis.app/v2";
"use strict";

nav("test result");
const main = document.getElementById("main");
const params = new URLSearchParams(location.search);

const tileState = t => {
	const ls = [...t.results];
	if (ls.some(l => FAILING.has(l))) return "fail";
	if (ls.some(l => l === "r")) return "refused";
	return "pass";
};

function tile(t, i) {
	const state = tileState(t);
	const glyph = { pass: icon("check", 'class="icon" style="fill:var(--accent)"'), fail: icon("cross", 'class="icon" style="fill:var(--state-error)"'), refused: icon("dash", 'class="icon" style="fill:var(--state-warn)"') }[state];
	const dots = [...t.results].map(l => `<i class="dot ${l}"></i>`).join("");
	return `<button class="tile rise ${state}" style="--i:${Math.min(i, 36)}" data-test="${esc(t.name)}" title="${esc(t.name)}">${glyph}<span class="name">${esc(t.name)}</span><span class="dots">${dots}</span></button>`;
}

function sectionView(section, si) {
	const failingTests = section.tests.filter(t => tileState(t) !== "pass");
	// big generated sections open on what went wrong; the hand-written cases show everything
	let filter = section.tests.length > 60 ? "failed" : "all";
	const el = document.createElement("section");
	el.className = "section rise";
	el.style.setProperty("--i", si * 3);
	const render = () => {
		const shown = filter === "all" ? section.tests : failingTests;
		const limit = el.dataset.expanded ? Infinity : 240;
		const tiles = shown.slice(0, limit).map(tile).join("");
		el.innerHTML = `
			<div class="section-head">
				<h2>${esc(section.title)}</h2>
				<span class="count">${section.score.passed} / ${section.score.total}</span>
				<span class="grow"></span>
				<div class="chips">
					<button class="chip ${filter === "all" ? "on" : ""}" data-f="all">all ${section.tests.length}</button>
					<button class="chip ${filter === "failed" ? "on" : ""}" data-f="failed">not passing ${failingTests.length}</button>
				</div>
			</div>
			<p class="section-about">${esc(section.about)}</p>
			${shown.length ? `<div class="grid">${tiles}</div>` : `<div class="empty">every program here passed at every profile.</div>`}
			${shown.length > limit ? `<button class="btn more" data-more>show all ${shown.length}</button>` : ""}`;
	};
	el.addEventListener("click", e => {
		const f = e.target.closest("[data-f]");
		if (f) { filter = f.dataset.f; render(); return; }
		if (e.target.closest("[data-more]")) { el.dataset.expanded = "1"; render(); return; }
		const t = e.target.closest("[data-test]");
		if (t) openTest(section, section.tests.find(x => x.name === t.dataset.test));
	});
	render();
	return el;
}

let current;
function openTest(section, t) {
	const d = document.getElementById("drawer"), s = document.getElementById("scrim");
	const rows = current.profiles.map((p, i) => {
		const l = t.results[i];
		return `<div class="prow"><span class="profile">${esc(p)}</span><span class="v ${l}">${VERDICT[l] || l}</span><span class="detail">${esc(t.details?.[p] || "")}</span></div>`;
	}).join("");
	const repro = section.id === "generated"
		? `rosetta gen ${t.name.replace("seed", "")}`
		: section.id === "cases" ? `cases/${t.name}.luau`
		: section.id === "libraries" ? `libraries/${t.name}/` : "rosetta mutate --count N --seed 1 --dump dir";
	d.innerHTML = `
		<div class="drawer-head"><h3>${esc(t.name)}</h3><button class="btn ghost" data-close title="close">${icon("close")}</button></div>
		<div class="drawer-body">
			${t.about ? `<p class="about keep">${esc(t.about)}</p>` : ""}
			${rows}
		</div>
		<div class="drawer-foot">the program: <code>${esc(repro)}</code>. <span class="keep">O</span> is the optimization level, <span class="keep">g</span> the debug level.</div>`;
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.set("test", t.name);
		history.replaceState(null, "", url);
	}
	s.classList.add("open"); d.classList.add("open");
	d.querySelector("[data-close]").focus();
}
function closeTest() {
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.delete("test");
		history.replaceState(null, "", url);
	}
	document.getElementById("drawer").classList.remove("open");
	document.getElementById("scrim").classList.remove("open");
}
document.getElementById("scrim").addEventListener("click", closeTest);
document.getElementById("drawer").addEventListener("click", e => e.target.closest("[data-close]") && closeTest());
addEventListener("keydown", e => e.key === "Escape" && closeTest());

const VERIFY_TEXT = {
	verified: ["verified test result", "an official run, published in the rosetta repo byte for byte."],
	unverified: ["unverified test result", "this file wasn't published by rosetta. it adds up, but anyone could have written it."],
	spoofed: ["spoofed test result", "this file contradicts itself: someone edited it after rosetta wrote it."],
};

function sidePanel(data, check, bytes, source) {
	const p = pct(data.score);
	const failed = failures(data.totals);
	const refused = data.totals.refused || 0;
	const [title, text] = check.signed ? ["verified test result", "an official run, signed with rosetta's key: byte for byte what the bench wrote."] : VERIFY_TEXT[check.state];
	const glyph = { verified: icon("check", 'style="fill:var(--accent)"'), unverified: icon("bang", 'style="fill:var(--state-warn)"'), spoofed: icon("warn", 'style="fill:var(--state-error)"') }[check.state];
	const problems = check.problems.length ? `<ul class="keep">${check.problems.slice(0, 6).map(p => `<li>${esc(p)}</li>`).join("")}</ul>` : "";
	const unknown = check.unknownSuite ? `<ul><li>its suite (${esc(data.suite?.hash)}) isn't one the site knows</li></ul>` : "";
	const verdictLine = failed
		? `<div class="verdict-line bad">${failed} wrong round trip${failed === 1 ? "" : "s"}${refused ? ` · ${refused} refused` : ""}</div>`
		: refused
			? `<div class="verdict-line" style="color:var(--state-warn)">nothing wrong · ${refused} refused</div>`
			: `<div class="verdict-line">no round trips failed!</div>`;
	const created = data.created ? new Date(data.created).toLocaleString() : "unknown";
	const q = data.quality;
	const quality = q ? `
		<div class="card">
			<h4>how the passing output reads</h4>
			<div class="meta">
				<div>similarity to the source<span>${q.similarity.toFixed(1)}%</span></div>
				<div>size vs the source<span>${q.size.toFixed(2)}×</span></div>
				<div>locals vs the source<span>${q.locals.toFixed(2)}×</span></div>
				<div>extra nesting<span>${q.nesting >= 0 ? "+" : ""}${q.nesting.toFixed(2)}</span></div>
				<div>endless loops added<span>${q.endless}</span></div>
				${q.names != null ? `<div>names recovered at g2<span>${q.names.toFixed(1)}%</span></div>` : ""}
				<div>median cpu per decompile<span>${q.cpu_ms_median.toFixed(1)} ms</span></div>
			</div>
			<p class="note">over this decompiler's own ${q.round_trips} passes. the readme compares decompilers on the passes they share.</p>
		</div>` : "";
	const bars = data.sections.map(s => `
		<div><div class="bar-label"><span>${esc(s.title)}</span><span>${s.score.passed} / ${s.score.total}</span></div>
		<div class="track2"><i data-w="${pct(s.score)}%"></i></div></div>`).join("");
	// the hard tier: how the rewritten bytecode went at each transform level
	const LEVEL = { 1: "registers, no-ops, junk", 2: "+ blocks, swaps", 3: "+ fake branches" };
	const levelOf = data.profiles.map(p => Number(/\+x(\d)/.exec(p)?.[1] || 0));
	let levelCard = "";
	if (levelOf.some(l => l)) {
		const by = {};
		for (const s of data.sections) for (const t of s.tests) [...t.results].forEach((v, j) => {
			const l = levelOf[j];
			if (!l || v === "u") return;
			by[l] ??= { passed: 0, total: 0 };
			by[l].total++;
			if (v === "p") by[l].passed++;
		});
		levelCard = `<div class="card"><h4>by transform level</h4><div class="bars">${Object.keys(by).sort().map(l => `
			<div><div class="bar-label"><span>x${l} · ${esc(LEVEL[l] || "")}</span><span>${by[l].passed} / ${by[l].total}</span></div>
			<div class="track2"><i data-w="${pct(by[l])}%"></i></div></div>`).join("")}</div>
			<p class="note">the bytecode each decompiler got was rewritten to do exactly the same thing at these levels: see the readme's hard tier.</p></div>`;
	}
	const shareable = check.state !== "verified" && !params.get("rubis");
	return `
		<div class="card edge ${check.state}">
			<div class="verify ${check.state}">${glyph}<div><h3>${title}</h3><p>${text}</p>${problems}${unknown}</div></div>
			${ring(p / 100, fmtPct(p))}
			<div class="score"><b>${data.score.passed.toLocaleString()}</b> / ${data.score.total.toLocaleString()}</div>
			${verdictLine}
			<div class="meta">
				<div>${icon("chip")}decompiler<span>${esc(data.decompiler.name)}${data.decompiler.version ? ` <span class="keep" style="color:var(--text-muted);font-weight:400">${esc(data.decompiler.version)}</span>` : ""}</span></div>
				<div>${icon("clock")}time taken (seconds)<span>${Number(data.seconds).toFixed(2)}</span></div>
				<div>${icon("layers")}suite<span>${esc(data.suite.name)} ${esc(data.suite.tier)}</span></div>
				<div>${icon("gear")}rosetta / luau<span>${esc(data.rosetta)} / ${esc(data.luau)}</span></div>
				<div>${icon("tag")}suite hash<span class="keep">${esc(data.suite.hash)}</span></div>
			</div>
		</div>
		<div class="card"><div class="meta"><div>${icon("cal")}created<span>${esc(created)}</span></div></div></div>
		<div class="card"><h4>by section</h4><div class="bars">${bars}</div>
			<div class="legend" style="margin-top:16px">
				<span><i class="dot p"></i>pass</span><span><i class="dot w"></i>wrong / doesn't recompile / crashed</span>
				<span><i class="dot r"></i>refused</span><span><i class="dot u"></i>unusable input</span>
			</div>
			<p class="note">each tile's bars are its profiles: <span class="keep">${data.profiles.map(esc).join(", ")}</span>.</p>
		</div>
		${levelCard}
		${quality}
		<button class="btn" data-download>${icon("download")}download json</button>
		${shareable ? `<button class="btn" data-share title="uploads this result to rubis.app, where anyone with the link can read it">${icon("link")}<span>upload &amp; copy link</span></button>` : ""}
		${params.get("rubis") ? `<button class="btn" data-copy>${icon("link")}<span>copy link</span></button>` : ""}
		<a class="btn" href="result">${icon("upload")}view another result</a>`;
}

async function show(bytes, source) {
	let data;
	try {
		data = JSON.parse(new TextDecoder().decode(bytes));
	} catch {
		return showPicker("that file isn't json.");
	}
	if (data?.format !== "rosetta-result") return showPicker("that json isn't a rosetta result. make one with rosetta bench.");
	current = data;
	const check = await verify(bytes, data);
	document.title = `${data.decompiler.name} · rosetta ${data.suite.tier}`;
	main.innerHTML = '<div class="result"><div class="sections"></div><aside class="side"></aside></div>';
	const sections = main.querySelector(".sections");
	data.sections.forEach((s, i) => sections.append(sectionView(s, i)));
	const side = main.querySelector(".side");
	side.innerHTML = sidePanel(data, check, bytes, source);
	side.classList.add("fade");
	animateIn(side);
	// ?test=name opens that program's drawer, so a single failure can be linked
	const wanted = params.get("test");
	for (const s of wanted ? data.sections : []) {
		const t = s.tests.find(x => x.name === wanted);
		if (t) { openTest(s, t); break; }
	}
	side.querySelector("[data-download]").addEventListener("click", () => {
		const a = document.createElement("a");
		a.href = URL.createObjectURL(new Blob([bytes], { type: "application/json" }));
		a.download = source?.file || `${data.decompiler.name}-${data.suite.tier}.json`;
		a.click();
	});
	// sharing uploads the file to Rubiš (public), and only when clicked
	side.querySelector("[data-share]")?.addEventListener("click", async e => {
		const label = e.currentTarget.querySelector("span");
		label.textContent = "uploading…";
		try {
			const title = encodeURIComponent(`rosetta ${data.suite.name} ${data.decompiler.name} ${data.suite.tier}`);
			const res = await fetch(`${RUBIS}/scrap?public=true&title=${title}`, { method: "POST", headers: { "Content-Type": "text/plain" }, body: bytes });
			const reply = await res.json();
			if (!res.ok || !/^[\w-]{1,64}$/.test(reply.scrapID || "")) throw new Error();
			const url = `${location.origin}${location.pathname}?rubis=${reply.scrapID}`;
			history.replaceState(null, "", url);
			await navigator.clipboard.writeText(url);
			label.textContent = "link copied";
		} catch {
			label.textContent = "upload failed, try again";
		}
	});
	side.querySelector("[data-copy]")?.addEventListener("click", async e => {
		await navigator.clipboard.writeText(location.href);
		e.currentTarget.querySelector("span").textContent = "link copied";
	});
}

async function showPicker(error) {
	document.title = "rosetta · view a result";
	main.innerHTML = `
		<div style="padding:30px 0 80px">
			<label class="drop rise" id="drop">
				${icon("upload")}
				<h2>drop a result here</h2>
				<p>the json <span class="keep"><code>rosetta bench</code></span> writes. it's read in your browser and only uploaded if you share it.</p>
				${error ? `<p style="color:var(--state-error)">${esc(error)}</p>` : ""}
				<input type="file" accept=".json,application/json" hidden>
				<span class="btn accent">choose a file</span>
			</label>
			<div class="list" id="official"></div>
		</div>`;
	const drop = document.getElementById("drop");
	const input = drop.querySelector("input");
	const take = async file => {
		history.replaceState(null, "", "result");
		show(new Uint8Array(await file.arrayBuffer()), { file: file.name });
	};
	input.addEventListener("change", () => input.files[0] && take(input.files[0]));
	drop.addEventListener("dragover", e => { e.preventDefault(); drop.classList.add("over"); });
	drop.addEventListener("dragleave", () => drop.classList.remove("over"));
	drop.addEventListener("drop", e => {
		e.preventDefault(); drop.classList.remove("over");
		e.dataTransfer.files[0] && take(e.dataTransfer.files[0]);
	});
	const index = await officialResults();
	if (index.results.length) {
		document.getElementById("official").innerHTML = `<div class="section-head rise" style="--i:2"><h2>official results</h2></div>` +
			index.results.map((r, i) => `<a class="tile rise pass" style="--i:${i + 3};height:46px" href="result?r=${encodeURIComponent(r.file.replace(/\.json$/, ""))}">
				${icon("check", 'class="icon" style="fill:var(--accent)"')}
				<span class="name" style="font-family:var(--ui)">${esc(r.decompiler)} <span class="keep" style="color:var(--text-dim)">${esc(r.version || "")}</span> · ${esc(r.tier)}</span>
				<span class="count" style="color:var(--text-muted);font-variant-numeric:tabular-nums">${fmtPct(pct(r.score))}</span></a>`).join("");
	}
}

async function start() {
	try {
		if (location.hash.startsWith("#z=")) return show(await unpack(location.hash.slice(3)), null);
		// a result shared through Rubiš (rubis.app): fetched from there, checked like any other
		const rubis = params.get("rubis");
		if (rubis) {
			if (!/^[\w-]{1,64}$/.test(rubis)) throw new Error("that isn't a rubiš scrap id");
			const res = await fetch(`${RUBIS}/scrap/${rubis}/raw`);
			if (!res.ok) throw new Error(`couldn't load scrap ${rubis} from rubiš (HTTP ${res.status})`);
			return show(new Uint8Array(await res.arrayBuffer()), { file: `${rubis}.json` });
		}
		const r = params.get("r"), url = params.get("url");
		if (r || url) {
			// a name, or <suite>/<name> for an archived result: nothing that climbs out of results/
			const path = r ? `results/${r.replace(/[^\w./-]/g, "").split("/").filter(p => p && !p.startsWith(".")).join("/")}.json` : url;
			const res = await fetch(path, { cache: "no-cache" });
			if (!res.ok) throw new Error(`couldn't load ${r || url} (HTTP ${res.status})`);
			return show(new Uint8Array(await res.arrayBuffer()), { file: path.split("/").pop() });
		}
		showPicker();
	} catch (e) {
		showPicker(e.message);
	}
}
start();

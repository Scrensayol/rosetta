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
// the failure gallery: <result>.gallery.json beside an official result, loaded the first time a
// failed test is opened. Only for results loaded by name or url; a dropped or shared file has none
let galleryUrl = null, galleryLoad = null;
function loadGallery() {
	if (!galleryUrl) return Promise.resolve(null);
	galleryLoad ??= fetch(galleryUrl, { cache: "no-cache" })
		.then(res => res.ok ? res.json() : null)
		.then(g => {
			// it has to be this result's: same suite build, same decompiler
			if (g?.format !== "rosetta-gallery" || g.suite?.hash !== current.suite.hash || g.decompiler !== current.decompiler.name) return null;
			g.byKey = new Map(g.entries.map(e => [`${e.test}\n${e.profile}`, e]));
			return g;
		})
		.catch(() => null);
	return galleryLoad;
}

const GX_HEAD = { mismatch: "wrong output", recompile_failed: "doesn't recompile", decompile_failed: "decompiler crashed", pass: "pass", refused: "refused" };
// the window of printed lines around the first difference; on that line, the point where the two
// sides part is marked
function gxLines(lines, other, from, first, side) {
	return lines.map((l, i) => {
		const n = from + i;
		if (n !== first) return `<span class="l"><span class="ln">${n}</span>${esc(l) || " "}</span>`;
		const o = other[i] ?? "";
		let k = 0;
		while (k < l.length && k < o.length && l[k] === o[k]) k++;
		return `<span class="l hit ${side}"><span class="ln">${n}</span>${esc(l.slice(0, k))}<mark>${esc(l.slice(k)) || " "}</mark></span>`;
	}).join("");
}
const lines = n => `${n.toLocaleString()} line${n === 1 ? "" : "s"}`;
function gxBlock(title, text, cls = "") {
	return `<h4><span>${title}</span><button class="btn ghost gx-copy" data-copy-text title="copy">${icon("copy")}</button></h4><pre class="${cls}">${esc(text)}</pre>`;
}
function galleryPanel(section, t, e, g) {
	const c = e.compare;
	const recorded = e.recorded
		? `<p class="gx-note">the result says ${esc(GX_HEAD[e.recorded] || e.recorded)}, but when the gallery was made this run ended as ${esc(GX_HEAD[e.kind] || e.kind)}: the decompiler may have changed since.</p>`
		: "";
	const printed = c && (c.want.length || c.got.length);
	const sub = !c ? "" : c.first ? `first difference on line ${c.first} · ${lines(c.want_lines)} vs ${lines(c.got_lines)}`
		: printed ? `every line matched: only how it ended differs · ${lines(c.want_lines)}` : "nothing printed: only how it ended differs";
	const cmp = c ? `
		<h4><span>what it printed</span><span class="gx-sub">${sub}</span></h4>
		<div class="gx-cmp">
			<div><div class="gx-label">the original</div>${printed ? `<pre>${gxLines(c.want, c.got, c.from, c.first, "want")}</pre>` : ""}<div class="gx-end${c.first ? "" : " hit"}">${esc(c.want_ending)}</div></div>
			<div><div class="gx-label">the decompiled output</div>${printed ? `<pre>${gxLines(c.got, c.want, c.from, c.first, "got")}</pre>` : ""}<div class="gx-end${c.first ? "" : " hit got"}">${esc(c.got_ending)}</div></div>
		</div>` : "";
	const why = e.kind === "recompile_failed" ? gxBlock("why it doesn't recompile", e.detail, "err")
		: e.kind === "decompile_failed" ? gxBlock("what the decompiler said", e.detail, "err") : "";
	const output = e.output != null ? gxBlock(`the decompiler's output${e.reduced ? " for the smaller program" : ""}`, e.output) : "";
	const program = e.reduced ? gxBlock("a smaller program it also gets wrong", e.reduced)
		: e.source != null ? gxBlock("the program", e.source)
		: section.id === "libraries" ? `<h4><span>the program</span></h4><p class="gx-sub gx-pad">the library and its spec are in <code>libraries/${esc(t.name)}/</code>.</p>` : "";
	return `
		<div class="gx-head"><span class="profile">${esc(e.profile)}</span><span class="v ${e.kind === "pass" ? "p" : "w"}">${esc(GX_HEAD[e.kind] || e.kind)}</span><span class="grow"></span>
			<button class="btn ghost" data-gx-close title="close">${icon("close")}</button></div>
		${recorded}${cmp}${why}${output}${program}
		<p class="gx-foot">${e.reduced ? "the program was shrunk while the decompiler kept getting it wrong (the same kind of failure, not always the same line), so what it printed is the smaller program's. " : ""}long text is cut at 16 kb.</p>`;
}

function openTest(section, t) {
	const d = document.getElementById("drawer"), s = document.getElementById("scrim");
	const rows = current.profiles.map((p, i) => {
		const l = t.results[i];
		return `<div class="prow" data-p="${esc(p)}"><span class="profile">${esc(p)}</span><span class="v ${l}">${VERDICT[l] || l}</span><span class="detail">${esc(t.details?.[p] || "")}</span><span class="act"></span></div>`;
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
			<div class="gx" id="gx" hidden></div>
		</div>
		<div class="drawer-foot">the program: <code>${esc(repro)}</code>. <span class="keep">O</span> is the optimization level, <span class="keep">g</span> the debug level.</div>`;
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.set("test", t.name);
		history.replaceState(null, "", url);
	}
	s.classList.add("open"); d.classList.add("open");
	d.classList.remove("wide");
	d.querySelector("[data-close]").focus();
	// failed rows with a gallery entry get a button that opens it
	if ([...t.results].some(l => FAILING.has(l))) loadGallery().then(g => {
		if (!g || d.dataset.test !== t.name) return;
		for (const row of d.querySelectorAll(".prow")) {
			const e = g.byKey.get(`${t.name}\n${row.dataset.p}`);
			if (e) row.querySelector(".act").innerHTML = `<button class="chip" data-g="${esc(row.dataset.p)}">see why</button>`;
		}
		const wanted = params.get("profile");
		if (wanted && g.byKey.has(`${t.name}\n${wanted}`)) showEntry(section, t, wanted);
	});
	d.dataset.test = t.name;
	d.onclick = e => {
		const b = e.target.closest("[data-g]");
		if (b) return showEntry(section, t, b.dataset.g);
		if (e.target.closest("[data-gx-close]")) return hideEntry();
		const copy = e.target.closest("[data-copy-text]");
		if (copy) navigator.clipboard.writeText(copy.closest("h4").nextElementSibling.textContent);
	};
}
async function showEntry(section, t, profile) {
	const g = await loadGallery();
	const e = g?.byKey.get(`${t.name}\n${profile}`);
	const d = document.getElementById("drawer"), panel = document.getElementById("gx");
	if (!e || !panel) return;
	for (const row of d.querySelectorAll(".prow")) row.classList.toggle("on", row.dataset.p === profile);
	panel.innerHTML = galleryPanel(section, t, e, g);
	panel.hidden = false;
	d.classList.add("wide");
	panel.scrollIntoView({ block: "nearest", behavior: matchMedia("(prefers-reduced-motion: reduce)").matches ? "auto" : "smooth" });
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.set("profile", profile);
		history.replaceState(null, "", url);
	}
}
function hideEntry() {
	const d = document.getElementById("drawer"), panel = document.getElementById("gx");
	panel.hidden = true;
	d.classList.remove("wide");
	for (const row of d.querySelectorAll(".prow")) row.classList.remove("on");
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.delete("profile");
		history.replaceState(null, "", url);
	}
}
function closeTest() {
	if (params.get("r")) {
		const url = new URL(location.href);
		url.searchParams.delete("test");
		url.searchParams.delete("profile");
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
	// from rosetta-4 on: of the passes, how many also made the same calls that change things
	const sc = data.same_calls;
	const callsLine = sc && sc.total ? `<div class="verdict-line" style="color:var(--text-dim)">same calls on ${sc.passed.toLocaleString()} of ${sc.total.toLocaleString()} passes</div>` : "";
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
	const obfuscated = data.profiles.some(p => /\+o\d/.test(p));
	const LEVEL = obfuscated ? { 1: "weak: flattened, constant array", 2: "medium: vm, encrypted strings", 3: "strong: all of it" } : { 1: "registers, no-ops, junk", 2: "+ blocks, swaps", 3: "+ fake branches" };
	const levelOf = data.profiles.map(p => Number(/\+[xo](\d)/.exec(p)?.[1] || 0));
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
		levelCard = `<div class="card"><h4>by ${obfuscated ? "obfuscation" : "transform"} level</h4><div class="bars">${Object.keys(by).sort().map(l => `
			<div><div class="bar-label"><span>x${l} · ${esc(LEVEL[l] || "")}</span><span>${by[l].passed} / ${by[l].total}</span></div>
			<div class="track2"><i data-w="${pct(by[l])}%"></i></div></div>`).join("")}</div>
			<p class="note">${obfuscated ? "each program was obfuscated as source at these levels before compiling: see the readme's obfuscated tier." : "the bytecode each decompiler got was rewritten to do exactly the same thing at these levels: see the readme's hard tier."}</p></div>`;
	}
	const shareable = check.state !== "verified" && !params.get("rubis");
	return `
		<div class="card edge ${check.state}">
			<div class="verify ${check.state}">${glyph}<div><h3>${title}</h3><p>${text}</p>${problems}${unknown}</div></div>
			${ring(p / 100, fmtPct(p))}
			<div class="score"><b>${data.score.passed.toLocaleString()}</b> / ${data.score.total.toLocaleString()}</div>
			${verdictLine}${callsLine}
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
			galleryUrl = path.replace(/\.json(?=$|[?#])/, ".gallery.json");
			return show(new Uint8Array(await res.arrayBuffer()), { file: path.split("/").pop() });
		}
		showPicker();
	} catch (e) {
		showPicker(e.message);
	}
}
start();

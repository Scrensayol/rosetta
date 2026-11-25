// shared by both pages: icons, the nav, the constellation, verification and loading
"use strict";

const REPO = "https://github.com/Scrensayol/rosetta";
const DISCORD = "https://discord.gg/7qE27wRwnd";

const ICON = {
	home: '<path d="M12 3 2 12h3v8h6v-6h2v6h6v-8h3z"/>',
	docs: '<path d="M6 2h9l5 5v13a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2zm8 1.5V8h4.5zM8 12v2h8v-2zm0 4v2h8v-2z"/>',
	github: '<path d="M12 2a10 10 0 0 0-3.16 19.49c.5.09.68-.22.68-.48v-1.7c-2.78.6-3.37-1.34-3.37-1.34-.45-1.16-1.11-1.47-1.11-1.47-.91-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.9 1.52 2.34 1.08 2.91.83.09-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.94 0-1.09.39-1.98 1.03-2.68-.1-.25-.45-1.27.1-2.65 0 0 .84-.27 2.75 1.02a9.6 9.6 0 0 1 5 0c1.91-1.29 2.75-1.02 2.75-1.02.55 1.38.2 2.4.1 2.65.64.7 1.03 1.59 1.03 2.68 0 3.84-2.34 4.68-4.57 4.93.36.31.68.92.68 1.85v2.74c0 .27.18.58.69.48A10 10 0 0 0 12 2z"/>',
	result: '<path d="M3 3h8v8H3zm10 0h8v8h-8zM3 13h8v8H3zm13 0h2v3h3v2h-3v3h-2v-3h-3v-2h3z"/>',
	info: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm1 15h-2v-6h2zm0-8h-2V7h2z"/>',
	check: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm-1.4 14.2-4.3-4.3 1.4-1.4 2.9 2.9 6-6 1.4 1.4z"/>',
	cross: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm4.2 12.8-1.4 1.4L12 13.4l-2.8 2.8-1.4-1.4 2.8-2.8-2.8-2.8 1.4-1.4 2.8 2.8 2.8-2.8 1.4 1.4-2.8 2.8z"/>',
	dash: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm5 11H7v-2h10z"/>',
	warn: '<path d="M12 2 1 21h22zm1 15h-2v-2h2zm0-4h-2V9h2z"/>',
	bang: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm1 15h-2v-2h2zm0-4h-2V7h2z"/>',
	clock: '<path d="M6 2h12v2h-1v3a5 5 0 0 1-2.4 4.3v1.4A5 5 0 0 1 17 17v3h1v2H6v-2h1v-3a5 5 0 0 1 2.4-4.3v-1.4A5 5 0 0 1 7 7V4H6zm3 2v3a3 3 0 0 0 6 0V4z"/>',
	gear: '<path d="M19.4 13a7.6 7.6 0 0 0 0-2l2.1-1.6-2-3.5-2.5 1a7.4 7.4 0 0 0-1.7-1L15 3h-4l-.4 2.9a7.4 7.4 0 0 0-1.7 1l-2.5-1-2 3.5L6.6 11a7.6 7.6 0 0 0 0 2l-2.1 1.6 2 3.5 2.5-1a7.4 7.4 0 0 0 1.7 1L11 21h4l.4-2.9a7.4 7.4 0 0 0 1.7-1l2.5 1 2-3.5zM13 15.5a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7z"/>',
	box: '<path d="M3 4h18v12H3zm2 2v8h14V6zm3 12h8v2H8z"/>',
	layers: '<path d="m12 2 10 5-10 5L2 7zm-7.6 8.2L12 14l7.6-3.8L22 11.4l-10 5-10-5zm0 4L12 18l7.6-3.8 2.4 1.2-10 5-10-5z"/>',
	tag: '<path d="M2 12V2h10l10 10-10 10zm5-5.5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3z"/>',
	cal: '<path d="M7 2h2v2h6V2h2v2h3v18H4V4h3zm-1 8v10h12V10z"/>',
	chip: '<path d="M9 2h2v2h2V2h2v2h2a2 2 0 0 1 2 2v2h2v2h-2v2h2v2h-2v2a2 2 0 0 1-2 2h-2v2h-2v-2h-2v2H9v-2H7a2 2 0 0 1-2-2v-2H3v-2h2v-2H3V8h2V6a2 2 0 0 1 2-2h2zm-1 6v8h8V8z"/>',
	upload: '<path d="M11 15V6.8L7.4 10.4 6 9l6-6 6 6-1.4 1.4L13 6.8V15zm-7 3h16v3H4z"/>',
	download: '<path d="M11 3h2v8.2l3.6-3.6L18 9l-6 6-6-6 1.4-1.4 3.6 3.6zm-7 15h16v3H4z"/>',
	link: '<path d="M10.6 13.4a1 1 0 0 1 0-1.4l3.5-3.5a1 1 0 1 1 1.4 1.4L12 13.4a1 1 0 0 1-1.4 0zM8.5 19.5a4 4 0 0 1-2.8-6.8l2.1-2.1 1.4 1.4-2.1 2.1a2 2 0 0 0 2.8 2.8l2.1-2.1 1.4 1.4-2.1 2.1a4 4 0 0 1-2.8 1.2zm7.7-6.1-1.4-1.4 2.1-2.1a2 2 0 0 0-2.8-2.8L12 9.2l-1.4-1.4 2.1-2.1a4 4 0 0 1 5.6 5.6z"/>',
	close: '<path d="m6.4 5 5.6 5.6L17.6 5 19 6.4 13.4 12l5.6 5.6-1.4 1.4-5.6-5.6L6.4 19 5 17.6l5.6-5.6L5 6.4z"/>',
	copy: '<path d="M8 3h11v13h-2V5H8zM5 7h10v14H5z"/>',
	down: '<path d="m12 16-7-7 1.4-1.4 5.6 5.6 5.6-5.6L19 9z"/>',
	flask: '<path d="M8 2h8v2h-1v5.2l5.4 9.4A2 2 0 0 1 18.7 21H5.3a2 2 0 0 1-1.7-2.4L9 9.2V4H8zm3 2v5.7L8.3 14h7.4L13 9.7V4z"/>',
	shield: '<path d="M12 2 4 5v6c0 5 3.4 9.5 8 11 4.6-1.5 8-6 8-11V5zm0 2.2V19.9c-3.4-1.3-6-4.9-6-8.9V6.4z"/>',
	shuffle: '<path d="M15 4h5v5l-1.8-1.8-3.6 3.6-1.4-1.4 3.6-3.6zM4 5.4 5.4 4 20 18.6 18.6 20zM13.6 15l1.4-1.4 3.2 3.2L20 15v5h-5l1.8-1.8z"/>',
	eye: '<path d="M12 5c5 0 9 4.5 10 7-1 2.5-5 7-10 7S3 14.5 2 12c1-2.5 5-7 10-7zm0 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8zm0 2a2 2 0 1 1 0 4 2 2 0 0 1 0-4z"/>',
	ban: '<path d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20zm0 2a8 8 0 0 1 6.3 12.9L7.1 5.7A8 8 0 0 1 12 4zM5.7 7.1l11.2 11.2A8 8 0 0 1 5.7 7.1z"/>',
	discord: '<path d="M20.317 4.37a19.79 19.79 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.865-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.74 19.74 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.1 13.1 0 0 1-1.872-.892.077.077 0 0 1-.008-.128c.126-.094.252-.192.372-.292a.074.074 0 0 1 .078-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.3 12.3 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.84 19.84 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.956-2.419 2.157-2.419 1.21 0 2.176 1.095 2.157 2.42 0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419 0-1.333.955-2.419 2.157-2.419 1.21 0 2.176 1.095 2.157 2.42 0 1.333-.946 2.418-2.157 2.418z"/>',
	open: '<path d="M4 4h7v2H6v12h12v-5h2v7H4zm9 0h7v7h-2V7.4l-6.3 6.3-1.4-1.4L16.6 6H13z"/>',
};

function icon(name, attrs = "") {
	return `<svg viewBox="0 0 24 24" aria-hidden="true" ${attrs}>${ICON[name]}</svg>`;
}

function esc(s) {
	return String(s ?? "").replace(/[&<>"']/g, c => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[c]);
}

function nav(here) {
	const el = document.querySelector("header.nav");
	el.innerHTML = `
		<a class="logo" href="./"><span><b>r</b>osetta</span><small>${esc(here)}</small></a>
		<nav class="nav-links">
			<a href="./" title="home">${icon("home")}</a>
			<a href="result.html" title="view a result">${icon("result")}</a>
			<a href="${REPO}#readme" title="docs" target="_blank" rel="noopener">${icon("docs")}</a>
			<a href="${REPO}" title="github" target="_blank" rel="noopener">${icon("github")}</a>
			<a href="${DISCORD}" title="discord" target="_blank" rel="noopener">${icon("discord")}</a>
		</nav>`;
}

// ---- motion ----
const easeOutQuint = t => 1 - Math.pow(1 - t, 5);
const reduced = matchMedia("(prefers-reduced-motion: reduce)").matches;

function countUp(el, to, { ms = 1100, delay = 150, format = v => Math.round(v) } = {}) {
	if (reduced) {
		el.textContent = format(to);
		return;
	}
	const start = performance.now() + delay;
	const step = now => {
		const t = Math.min(1, Math.max(0, (now - start) / ms));
		el.textContent = format(to * easeOutQuint(t));
		if (t < 1) requestAnimationFrame(step);
	};
	el.textContent = format(0);
	requestAnimationFrame(step);
}

function ring(fraction, label, cls = "ring") {
	const r = 80, c = 2 * Math.PI * r;
	return `<svg class="${cls}" viewBox="0 0 190 190">
		<circle class="track" cx="95" cy="95" r="${r}"/>
		<circle class="arc" cx="95" cy="95" r="${r}" transform="rotate(-90 95 95)" stroke-dasharray="${c}" stroke-dashoffset="${c}" data-target="${c * (1 - fraction)}"/>
		<text x="95" y="97">${esc(label)}</text>
	</svg>`;
}

// starts every ring and bar under root once it's on screen
function animateIn(root) {
	const go = () => {
		root.querySelectorAll(".arc[data-target]").forEach(a => a.setAttribute("stroke-dashoffset", a.dataset.target));
		root.querySelectorAll(".track2 i[data-w]").forEach(i => (i.style.width = i.dataset.w));
	};
	// two frames, so the starting state is painted and the transition has something to run from
	if (reduced) go();
	else requestAnimationFrame(() => requestAnimationFrame(go));
}

// ---- the constellation behind the hero: drifting points, near ones joined ----
function constellation(canvas) {
	const ctx = canvas.getContext("2d");
	let w, h, points, visible = true;
	const resize = () => {
		const dpr = Math.min(devicePixelRatio || 1, 2);
		w = canvas.clientWidth; h = canvas.clientHeight;
		canvas.width = w * dpr; canvas.height = h * dpr;
		ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
		const n = Math.round(Math.min(110, (w * h) / 15000));
		points = Array.from({ length: n }, () => ({
			x: Math.random() * w, y: Math.random() * h,
			vx: (Math.random() - 0.5) * 0.16, vy: (Math.random() - 0.5) * 0.16,
		}));
	};
	const reach = 150;
	const draw = () => {
		ctx.clearRect(0, 0, w, h);
		for (const p of points) {
			if (!reduced) {
				p.x += p.vx; p.y += p.vy;
				if (p.x < 0 || p.x > w) p.vx *= -1;
				if (p.y < 0 || p.y > h) p.vy *= -1;
			}
		}
		ctx.lineWidth = 1;
		for (let i = 0; i < points.length; i++) {
			const a = points[i];
			for (let j = i + 1; j < points.length; j++) {
				const b = points[j], d = Math.hypot(a.x - b.x, a.y - b.y);
				if (d < reach) {
					ctx.strokeStyle = `rgba(255,138,180,${0.13 * (1 - d / reach)})`;
					ctx.beginPath(); ctx.moveTo(a.x, a.y); ctx.lineTo(b.x, b.y); ctx.stroke();
				}
			}
			ctx.fillStyle = "rgba(242,239,246,0.28)";
			ctx.fillRect(a.x - 1, a.y - 1, 2, 2);
		}
	};
	const loop = () => {
		if (visible) draw();
		if (!reduced) requestAnimationFrame(loop);
	};
	new IntersectionObserver(([e]) => (visible = e.isIntersecting)).observe(canvas);
	addEventListener("resize", () => { resize(); draw(); });
	resize();
	loop();
}

// ---- results ----
const VERDICT = {
	p: "pass", w: "wrong output", c: "doesn't recompile", f: "decompiler crashed", r: "refused", u: "unusable input",
};
const FAILING = new Set(["w", "c", "f"]);

async function fetchJSON(path) {
	const res = await fetch(path, { cache: "no-cache" });
	if (!res.ok) throw new Error(`${path}: HTTP ${res.status}`);
	return res.json();
}

let officialIndex;
function officialResults() {
	officialIndex ??= fetchJSON("results/index.json").catch(() => ({ results: [], suites: {} }));
	return officialIndex;
}

async function sha256(bytes) {
	if (!crypto.subtle) return null;
	const digest = await crypto.subtle.digest("SHA-256", bytes);
	return [...new Uint8Array(digest)].map(b => b.toString(16).padStart(2, "0")).join("");
}

function pct(s) {
	return s.total ? (s.passed / s.total) * 100 : 0;
}

function fmtPct(v) {
	return v >= 99.95 && v < 100 ? "99.9%" : `${v.toFixed(v === 100 ? 0 : 1)}%`;
}

function failures(totals) {
	return (totals.mismatch || 0) + (totals.recompile_failed || 0) + (totals.decompile_failed || 0);
}

// Is the file what it says it is? Verified: byte for byte an official run listed in
// results/index.json. Spoofed: its own numbers don't add up, or it claims a known suite but has
// different programs. Unverified: everything else that parses
async function verify(bytes, data) {
	const index = await officialResults();
	const problems = [];
	const hash = await sha256(bytes);
	const official = hash && index.results.find(r => r.sha256 === hash);
	if (official) return { state: "verified", official, problems };

	if (data.format !== "rosetta-result") problems.push("not a rosetta result file");
	if (data.schema !== 1) problems.push(`unknown schema ${data.schema}`);
	const profiles = data.profiles || [];
	const totals = {};
	const letterKind = data.letters || {};
	let overall = { passed: 0, total: 0 };
	for (const section of data.sections || []) {
		const score = { passed: 0, total: 0 };
		for (const t of section.tests || []) {
			if (typeof t.results !== "string" || t.results.length !== profiles.length) {
				problems.push(`${t.name}: ${t.results?.length} verdicts for ${profiles.length} profiles`);
				continue;
			}
			for (const l of t.results) {
				if (!(l in VERDICT)) { problems.push(`${t.name}: unknown verdict '${l}'`); continue; }
				const kind = letterKind[l] || l;
				totals[kind] = (totals[kind] || 0) + 1;
				if (l !== "u") score.total++;
				if (l === "p") score.passed++;
			}
		}
		if (section.score?.passed !== score.passed || section.score?.total !== score.total)
			problems.push(`${section.title}: claims ${section.score?.passed}/${section.score?.total}, its tests add up to ${score.passed}/${score.total}`);
		overall.passed += score.passed;
		overall.total += score.total;
	}
	if (data.score?.passed !== overall.passed || data.score?.total !== overall.total)
		problems.push(`claims ${data.score?.passed}/${data.score?.total}, its tests add up to ${overall.passed}/${overall.total}`);
	for (const [kind, n] of Object.entries(data.totals || {}))
		if ((totals[kind] || 0) !== n) problems.push(`claims ${n} ${kind.replace("_", " ")}, counts ${totals[kind] || 0}`);

	const suite = index.suites[data.suite?.hash];
	if (suite) {
		for (const section of data.sections || []) {
			const want = suite.sections[section.id];
			const names = (section.tests || []).map(t => t.name).join("\n");
			if (want && want.names !== undefined && want.names !== (await sha256(new TextEncoder().encode(names))))
				problems.push(`${section.title}: not the programs suite ${data.suite.hash} runs`);
		}
	}
	if (problems.length) return { state: "spoofed", problems };
	return { state: "unverified", problems, unknownSuite: !suite };
}

// share links for files that aren't official: the JSON, deflated, in the URL fragment
async function pack(bytes) {
	const stream = new Blob([bytes]).stream().pipeThrough(new CompressionStream("deflate-raw"));
	const buf = new Uint8Array(await new Response(stream).arrayBuffer());
	let s = "";
	for (let i = 0; i < buf.length; i += 0x8000) s += String.fromCharCode(...buf.subarray(i, i + 0x8000));
	return btoa(s).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

async function unpack(text) {
	const bin = atob(text.replace(/-/g, "+").replace(/_/g, "/"));
	const buf = Uint8Array.from(bin, c => c.charCodeAt(0));
	const stream = new Blob([buf]).stream().pipeThrough(new DecompressionStream("deflate-raw"));
	return new Uint8Array(await new Response(stream).arrayBuffer());
}

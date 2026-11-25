// The transform tier: rewrites a program's bytecode so it does exactly what it did, but no longer
// looks like what Luau's compiler writes. Decompilers that pattern-match the compiler's shapes
// break; ones that analyze control flow and data flow don't. Every rewrite is seeded by the
// bytecode itself, so every decompiler gets the same bytes, and the round trip runs the result
// before handing it over: a rewrite that changes what the program prints is Rosetta's bug, and
// the program counts for nobody.
//
// Levels (the x in a profile like O2g1+x2), each harder than the last:
// 1. registers past the parameters shifted up, so locals no longer start right after them;
//    no-op instructions and jumps to the next instruction; unreachable junk after the
//    instructions that never fall through. Nothing goes between a call whose results run to
//    the top of the stack and the instruction that takes them
// 2. and those can go there too, and basic blocks are laid out in a shuffled order, glued with
//    jumps. Debug ranges stop meaning anything once blocks move, so locals' names and types go
// 3. and branches that always go one way guard junk: a scratch register loaded with a boolean
//    and tested, or b <= a * a worked out at run time; functions are shuffled in the chunk, and
//    upvalue names go too
//
// ROSETTA_TRANSFORM_OFF=shift,insert,multret,junk,nop,jumpnext,blocks,branches,arith,protos,names
// turns pieces off, and ROSETTA_TRANSFORM_ONLY=a-b rewrites only those functions, to find what
// breaks a program or a decompiler

use std::collections::HashMap;

use crate::{
    chunk::{Chunk, Constant, Feedback, Proto},
    rng::Rng,
};

pub const LEVELS: u8 = 3;
// Bump when a rewrite changes: the hard tier's fingerprint includes it
pub const VERSION: &str = "transform-1";

mod op {
    pub const NOP: u8 = 0;
    pub const LOADNIL: u8 = 2;
    pub const LOADB: u8 = 3;
    pub const LOADN: u8 = 4;
    pub const LOADK: u8 = 5;
    pub const MOVE: u8 = 6;
    pub const NAMECALL: u8 = 20;
    pub const CALL: u8 = 21;
    pub const RETURN: u8 = 22;
    pub const JUMP: u8 = 23;
    pub const JUMPBACK: u8 = 24;
    pub const JUMPIF: u8 = 25;
    pub const JUMPIFNOT: u8 = 26;
    pub const JUMPIFLE: u8 = 28;
    pub const ADD: u8 = 33;
    pub const SUB: u8 = 34;
    pub const MUL: u8 = 35;
    pub const CONCAT: u8 = 49;
    pub const NOT: u8 = 50;
    pub const SETLIST: u8 = 55;
    pub const FORNPREP: u8 = 56;
    pub const FORNLOOP: u8 = 57;
    pub const FORGLOOP: u8 = 58;
    pub const FORGPREP_INEXT: u8 = 59;
    pub const FASTCALL3: u8 = 60;
    pub const FORGPREP_NEXT: u8 = 61;
    pub const GETVARARGS: u8 = 63;
    pub const PREPVARARGS: u8 = 65;
    pub const JUMPX: u8 = 67;
    pub const FASTCALL: u8 = 68;
    pub const COVERAGE: u8 = 69;
    pub const CAPTURE: u8 = 70;
    pub const SUBRK: u8 = 71;
    pub const DIVRK: u8 = 72;
    pub const FASTCALL1: u8 = 73;
    pub const FASTCALL2: u8 = 74;
    pub const FASTCALL2K: u8 = 75;
    pub const FORGPREP: u8 = 76;
    pub const IDIV: u8 = 81;
    pub const IDIVK: u8 = 82;
    pub const FASTPCALL: u8 = 89;
}

// BytecodeUtils.h's getOpLength: these carry an AUX word
fn has_aux(op: u8) -> bool {
    matches!(op, 7 | 8 | 12 | 15 | 16 | 20 | 27..=32 | 53 | 55 | 58 | 60 | 66 | 74 | 75 | 77..=80 | 83..=88 | 90 | 91 | 93 | 94)
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    D,
    E,
    C,
}

// where an instruction keeps a jump, always relative to the word after its first
fn jump_field(op: u8) -> Option<Field> {
    match op {
        23..=32 | 56..=59 | 61 | 76 | 77..=80 => Some(Field::D),
        op::JUMPX => Some(Field::E),
        op::LOADB | op::FASTCALL | op::FASTCALL1 | op::FASTCALL2 | op::FASTCALL2K | op::FASTCALL3 | op::FASTPCALL => Some(Field::C),
        _ => None,
    }
}

// (FASTPCALL is pcall's: A says whether the call's results start with the status)
fn is_fastcall(op: u8) -> bool {
    matches!(op, op::FASTCALL | op::FASTCALL1 | op::FASTCALL2 | op::FASTCALL2K | op::FASTCALL3 | op::FASTPCALL)
}

// never falls through to the next instruction
fn ends_block(op: u8) -> bool {
    matches!(op, op::JUMP | op::JUMPBACK | op::JUMPX | op::RETURN)
}

// native calls, userdata fast paths, feedback slots and user-defined classes: Rosetta's compiler
// settings never emit them, so they aren't modelled
fn unsupported(op: u8) -> bool {
    op == 62 || (op >= 83 && op != op::FASTPCALL)
}

#[derive(Clone)]
struct Ins {
    op: u8,
    word: u32,
    aux: Option<u32>,
    // the instruction this one jumps to, by index in the list it's in
    target: Option<usize>,
    // only in a block being inserted: a jump to the block's own instruction at this index, where
    // an index past the block means the instruction the block goes in front of
    local: Option<usize>,
    lines: (i32, i32),
    // the index this instruction had when decoded (None: added since)
    orig: Option<usize>,
}

impl Ins {
    fn new(op: u8, a: u8, b: u8, c: u8, line: i32) -> Ins {
        let word = op as u32 | (a as u32) << 8 | (b as u32) << 16 | (c as u32) << 24;
        Ins { op, word, aux: None, target: None, local: None, lines: (line, line), orig: None }
    }

    fn ad(op: u8, a: u8, d: i16, line: i32) -> Ins {
        let word = op as u32 | (a as u32) << 8 | ((d as u16 as u32) << 16);
        Ins { op, word, aux: None, target: None, local: None, lines: (line, line), orig: None }
    }

    fn jump(op: u8, a: u8, target: usize, line: i32) -> Ins {
        Ins { target: Some(target), ..Ins::new(op, a, 0, 0, line) }
    }

    fn local_jump(op: u8, a: u8, to: usize, line: i32) -> Ins {
        Ins { local: Some(to), ..Ins::new(op, a, 0, 0, line) }
    }

    fn a(&self) -> u8 {
        (self.word >> 8) as u8
    }

    fn b(&self) -> u8 {
        (self.word >> 16) as u8
    }

    fn c(&self) -> u8 {
        (self.word >> 24) as u8
    }

    fn len(&self) -> usize {
        1 + self.aux.is_some() as usize
    }
}

struct Func {
    ins: Vec<Ins>,
    // word pc -> instruction index, as decoded
    starts: HashMap<usize, usize>,
    size: usize,
}

fn decode(p: &Proto) -> Result<Func, String> {
    let code = &p.code;
    let line = |w: usize| p.lines.as_ref().map_or(0, |l| l[w]);
    let mut ins = Vec::new();
    let mut starts = HashMap::new();
    let mut at = Vec::new();
    let mut w = 0;
    while w < code.len() {
        let word = code[w];
        let op = word as u8;
        if unsupported(op) {
            return Err(format!("opcode {op} isn't supported"));
        }
        let aux = if has_aux(op) { Some(*code.get(w + 1).ok_or("an AUX word past the end")?) } else { None };
        starts.insert(w, ins.len());
        at.push(w);
        let lines = (line(w), if aux.is_some() { line(w + 1) } else { line(w) });
        ins.push(Ins { op, word, aux, target: None, local: None, lines, orig: Some(ins.len()) });
        w += 1 + aux.is_some() as usize;
    }
    for k in 0..ins.len() {
        if let Some(field) = jump_field(ins[k].op) {
            let off = match field {
                Field::D => (ins[k].word as i32) >> 16,
                Field::E => (ins[k].word as i32) >> 8,
                Field::C => ins[k].c() as i32,
            };
            let t = at[k] as i64 + 1 + off as i64;
            let t = usize::try_from(t).ok().and_then(|t| starts.get(&t)).ok_or_else(|| format!("the jump at {} lands inside an instruction", at[k]))?;
            ins[k].target = Some(*t);
        }
    }
    Ok(Func { ins, starts, size: code.len() })
}

// lays the instructions out: (words, each word's line, each instruction's pc)
fn encode(ins: &[Ins]) -> Result<(Vec<u32>, Vec<i32>, Vec<usize>), String> {
    let mut at = Vec::with_capacity(ins.len());
    let mut w = 0;
    for i in ins {
        at.push(w);
        w += i.len();
    }
    let mut code = Vec::with_capacity(w);
    let mut lines = Vec::with_capacity(w);
    for (k, i) in ins.iter().enumerate() {
        let mut word = i.word;
        if let Some(t) = i.target {
            let off = at[t] as i64 - (at[k] as i64 + 1);
            word = match jump_field(i.op).ok_or("a target on an instruction that doesn't jump")? {
                Field::D if (i16::MIN as i64..=i16::MAX as i64).contains(&off) => (word & 0xffff) | ((off as i16 as u16 as u32) << 16),
                Field::E if (-(1i64 << 23)..(1i64 << 23)).contains(&off) => (word & 0xff) | ((off as i32 as u32) << 8),
                Field::C if (0..=255).contains(&off) => (word & 0x00ff_ffff) | ((off as u32) << 24),
                _ => return Err(format!("a jump of {off} doesn't fit")),
            };
        }
        code.push(word);
        lines.push(i.lines.0);
        if let Some(aux) = i.aux {
            code.push(aux);
            lines.push(i.lines.1);
        }
    }
    Ok((code, lines, at))
}

// Where a rewrite may put instructions of its own, as "before instruction k". plain: anything
// that writes no register. writes: also a scratch register, which isn't safe while a call's or
// a vararg's results run to the top of the stack (they may reach past the frame's registers)
struct Spots {
    plain: Vec<bool>,
    writes: Vec<bool>,
}

fn spots(ins: &[Ins]) -> Spots {
    let n = ins.len();
    let mut plain = vec![true; n + 1];
    // (the function's first instruction may be PREPVARARGS, which has to stay first)
    plain[0] = false;
    let mut writes = vec![true; n + 1];
    let mut open = false;
    for k in 0..n {
        let i = &ins[k];
        // the captures that follow a closure, and the call that follows NAMECALL
        if i.op == op::CAPTURE || (k > 0 && ins[k - 1].op == op::NAMECALL) {
            plain[k] = false;
        }
        // from a fast call to its fallback call, inclusive
        if is_fastcall(i.op)
            && let Some(t) = i.target
        {
            for p in k + 1..=t.min(n) {
                plain[p] = false;
            }
        }
        if open {
            writes[k] = false;
        }
        let consumes = matches!(i.op, op::CALL if i.b() == 0) || matches!(i.op, op::RETURN if i.b() == 0) || matches!(i.op, op::SETLIST if i.c() == 0);
        let produces = matches!(i.op, op::CALL if i.c() == 0) || matches!(i.op, op::GETVARARGS if i.b() == 0);
        if consumes {
            open = false;
        }
        if produces {
            open = true;
        }
    }
    for k in 0..=n {
        writes[k] &= plain[k];
    }
    Spots { plain, writes }
}

// a few instructions that never run: they read and write real registers, jump into the real
// code and return, so they look like code
fn junk(rng: &mut Rng, regs: u8, consts: &[usize], targets: &[usize], line: i32) -> Vec<Ins> {
    let reg = |rng: &mut Rng| rng.below(regs.max(1) as usize) as u8;
    let mut out = Vec::new();
    for _ in 0..rng.range(1, 4) {
        let i = match rng.below(8) {
            0 => Ins::ad(op::LOADN, reg(rng), rng.range(-50, 50) as i16, line),
            1 if !consts.is_empty() => Ins::ad(op::LOADK, reg(rng), *rng.pick(consts) as i16, line),
            2 => Ins::new(op::MOVE, reg(rng), reg(rng), 0, line),
            3 => Ins::new(*rng.pick(&[op::ADD, op::SUB, op::MUL]), reg(rng), reg(rng), reg(rng), line),
            4 => Ins::new(op::NOT, reg(rng), reg(rng), 0, line),
            5 => Ins::new(op::LOADNIL, reg(rng), 0, 0, line),
            6 if !targets.is_empty() => Ins::jump(*rng.pick(&[op::JUMPIF, op::JUMPIFNOT]), reg(rng), *rng.pick(targets), line),
            // (C = 0: a LOADB that skips nothing)
            _ => Ins::new(op::LOADB, reg(rng), rng.below(2) as u8, 0, line),
        };
        out.push(i);
    }
    if !targets.is_empty() && rng.chance(50) {
        out.push(Ins::jump(op::JUMP, 0, *rng.pick(targets), line));
    } else {
        out.push(Ins::new(op::RETURN, 0, 1, 0, line));
    }
    out
}

// Inserts make(k) in front of every instruction k that want names. The original instructions'
// jumps keep their targets (the same instructions); an inserted one names an original by target
// or one of its own block's by local
fn insert(ins: Vec<Ins>, want: &[bool], mut make: impl FnMut(usize) -> Vec<Ins>) -> Vec<Ins> {
    enum Fix {
        Orig(usize),
        At(usize),
    }
    let n = ins.len();
    let mut out = Vec::with_capacity(n * 2);
    let mut fixes = Vec::new();
    let mut index = vec![0; n + 1];
    for (k, i) in ins.into_iter().enumerate() {
        if want[k] {
            let block = make(k);
            let (start, len) = (out.len(), block.len());
            for mut b in block {
                if let Some(l) = b.local.take() {
                    fixes.push((out.len(), if l >= len { Fix::Orig(k) } else { Fix::At(start + l) }));
                } else if let Some(t) = b.target {
                    fixes.push((out.len(), Fix::Orig(t)));
                }
                out.push(b);
            }
        }
        index[k] = out.len();
        if let Some(t) = i.target {
            fixes.push((out.len(), Fix::Orig(t)));
        }
        out.push(i);
    }
    index[n] = out.len();
    for (at, fix) in fixes {
        out[at].target = Some(match fix {
            Fix::Orig(t) => index[t],
            Fix::At(p) => p,
        });
    }
    out
}

// The registers an instruction names (bit offsets in its word, and in its AUX word), and the run
// of registers it reaches from its first one: (first, how many; -1 for "to the top")
#[allow(clippy::type_complexity)]
fn reg_fields(i: &Ins) -> (Vec<u32>, Vec<u32>, Option<(u8, i32)>) {
    let (a, b, c) = (8, 16, 24);
    let o = i.op;
    let mut main = Vec::new();
    let mut aux = Vec::new();
    // A is a register except where it's an id, a flag or unused
    let a_is_reg = !matches!(
        o,
        op::NOP | 1 | op::JUMP | op::JUMPBACK | op::PREPVARARGS | op::JUMPX | op::COVERAGE | op::CAPTURE | op::FASTCALL | op::FASTCALL1 | op::FASTCALL2 | op::FASTCALL2K | op::FASTCALL3 | op::FASTPCALL
    );
    if a_is_reg {
        main.push(a);
    }
    match o {
        op::MOVE | 13..=18 | op::NAMECALL | 39..=44 | op::IDIVK | 47 | 48 | 50..=52 | op::SETLIST => main.push(b),
        33..=38 | op::IDIV | 45 | 46 | op::CONCAT => main.extend([b, c]),
        op::SUBRK | op::DIVRK => main.push(c),
        op::FASTCALL1 | op::FASTCALL2 | op::FASTCALL2K | op::FASTCALL3 => main.push(b),
        // a captured value or reference (not an upvalue)
        op::CAPTURE if i.a() <= 1 => main.push(b),
        _ => {}
    }
    // GETTABLE and SETTABLE (13, 14) also index with a register
    if matches!(o, 13 | 14) {
        main.push(c);
    }
    match o {
        27..=32 | op::FASTCALL2 => aux.push(0),
        op::FASTCALL3 => aux.extend([0, 8]),
        _ => {}
    }
    let range = match o {
        op::CALL => {
            let (b, c) = (i.b() as i32, i.c() as i32);
            if b == 0 || c == 0 { Some((i.a(), -1)) } else { Some((i.a(), (b - 1).max(c - 1) + 1)) }
        }
        op::RETURN | op::GETVARARGS => Some((i.a(), if i.b() == 0 { -1 } else { i.b() as i32 - 1 })),
        op::SETLIST => Some((i.b(), if i.c() == 0 { -1 } else { i.c() as i32 - 1 })),
        op::CONCAT => Some((i.b(), i.c() as i32 - i.b() as i32 + 1)),
        op::NAMECALL => Some((i.a(), 2)),
        op::FORNPREP | op::FORNLOOP => Some((i.a(), 3)),
        op::FORGLOOP => Some((i.a(), 3 + (i.aux.unwrap_or(1) & 0xff) as i32)),
        op::FORGPREP | op::FORGPREP_INEXT | op::FORGPREP_NEXT => Some((i.a(), 5)),
        _ => None,
    };
    (main, aux, range)
}

// moves every register past the parameters up by k
fn shift_registers(ins: &mut [Ins], p: &mut Proto, k: u8) -> bool {
    let np = p.num_params;
    if k == 0 || p.max_stack as u32 + k as u32 > 240 {
        return false;
    }
    // a run that starts among the parameters and reaches past them can't come apart
    for i in ins.iter() {
        if let (_, _, Some((start, count))) = reg_fields(i)
            && start < np
            && (count < 0 || start as i32 + count > np as i32)
        {
            return false;
        }
    }
    for i in ins.iter_mut() {
        let (main, aux, _) = reg_fields(i);
        for s in main {
            let r = (i.word >> s) as u8;
            if r >= np {
                i.word = (i.word & !(0xff << s)) | ((r + k) as u32) << s;
            }
        }
        if let Some(mut x) = i.aux {
            for s in aux {
                let r = (x >> s) as u8;
                if r >= np {
                    x = (x & !(0xff << s)) | ((r + k) as u32) << s;
                }
            }
            i.aux = Some(x);
        }
    }
    p.max_stack += k;
    if let Some(d) = &mut p.debug {
        for l in d.locals.iter_mut().filter(|l| l.reg >= np) {
            l.reg += k;
        }
    }
    if let Some(t) = &mut p.types {
        for l in t.locals.iter_mut().filter(|l| l.reg >= np) {
            l.reg += k;
        }
    }
    true
}

// Splits the code into basic blocks and lays them out in a shuffled order, glued with jumps
// where a block used to fall into the next. What has to stay together (a fast call and its
// fallback call, a LOADB and what it skips to, a closure and its captures, NAMECALL and its
// call) moves as one, and the first block stays first: that's where the function starts
fn shuffle_blocks(ins: Vec<Ins>, rng: &mut Rng) -> Vec<Ins> {
    let n = ins.len();
    let mut leader = vec![false; n + 1];
    let mut glued = vec![false; n + 1];
    for (k, i) in ins.iter().enumerate() {
        if let Some(t) = i.target {
            if is_fastcall(i.op) || i.op == op::LOADB {
                for p in k + 1..=t.min(n) {
                    glued[p] = true;
                }
            } else {
                leader[t] = true;
                leader[k + 1] = true;
            }
        }
        if ends_block(i.op) {
            leader[k + 1] = true;
        }
        if i.op == op::CAPTURE || (k > 0 && ins[k - 1].op == op::NAMECALL) {
            glued[k] = true;
        }
    }
    let mut groups = Vec::new();
    let mut start = 0;
    for k in 1..=n {
        if k == n || (leader[k] && !glued[k]) {
            groups.push((start, k));
            start = k;
        }
    }
    if groups.len() < 3 {
        return ins;
    }
    let mut order = (1..groups.len()).collect::<Vec<_>>();
    for k in (1..order.len()).rev() {
        order.swap(k, rng.below(k + 1));
    }
    order.insert(0, 0);
    let mut out = Vec::with_capacity(n + groups.len());
    let mut index = vec![0; n + 1];
    for (pos, &g) in order.iter().enumerate() {
        let (s, e) = groups[g];
        for (k, i) in ins.iter().enumerate().take(e).skip(s) {
            index[k] = out.len();
            out.push(i.clone());
        }
        // falling off the end has to reach what came next before
        let last = &ins[e - 1];
        if e < n && !ends_block(last.op) && order.get(pos + 1) != Some(&(g + 1)) {
            out.push(Ins::jump(op::JUMP, 0, e, last.lines.1));
        }
    }
    index[n] = out.len();
    for i in &mut out {
        if let Some(t) = i.target {
            i.target = Some(index[t]);
        }
    }
    out
}

fn off(piece: &str) -> bool {
    std::env::var("ROSETTA_TRANSFORM_OFF").is_ok_and(|v| v.split(',').any(|p| p == piece))
}

fn transform_proto(p: &mut Proto, rng: &mut Rng, level: u8) -> Result<(), String> {
    let Func { mut ins, starts, size } = decode(p)?;
    // constants LOADK may load: nil, booleans, numbers, strings and vectors
    let consts = p.constants.iter().enumerate().filter(|(k, c)| *k < 32768 && matches!(c, Constant::Nil | Constant::Boolean(_) | Constant::Number(_) | Constant::String(_) | Constant::Vector(_))).map(|(k, _)| k).collect::<Vec<_>>();

    // level 1: registers up; no-ops, jumps to the next instruction, and junk after the
    // instructions that never fall through (inside an open result list from level 2)
    let shift = rng.range(1, 6) as u8;
    if !off("shift") {
        shift_registers(&mut ins, p, shift);
    }
    let regs = p.max_stack;
    let s = spots(&ins);
    let originals = (0..ins.len()).filter(|&k| s.plain[k]).collect::<Vec<_>>();
    let lines = ins.iter().map(|i| i.lines.0).collect::<Vec<_>>();
    let after_end = (0..ins.len()).map(|k| k > 0 && ends_block(ins[k - 1].op)).collect::<Vec<_>>();
    let want = (0..ins.len()).map(|k| s.plain[k] && rng.chance(if after_end[k] { 60 } else { 25 }) && !off("insert") && (s.writes[k] || (level >= 2 && !off("multret")))).collect::<Vec<_>>();
    let mut r = Rng::new(rng.next());
    ins = insert(ins, &want, |k| {
        let line = lines[k];
        let pick = r.chance(50);
        if after_end[k] {
            if off("junk") { Vec::new() } else { junk(&mut r, regs, &consts, &originals, line) }
        } else if pick {
            if off("nop") { Vec::new() } else { vec![Ins::new(op::NOP, 0, 0, 0, line)] }
        } else if off("jumpnext") {
            Vec::new()
        } else {
            vec![Ins::local_jump(op::JUMP, 0, 1, line)]
        }
    });

    // level 3: branches that always go one way guarding junk, testing scratch registers past
    // every other (blocks move at level 2, below)
    if level >= 3 {
        if p.max_stack <= 250 && !off("branches") {
            let scratch = p.max_stack;
            p.max_stack += 2;
            let s = spots(&ins);
            let originals = (0..ins.len()).filter(|&k| s.plain[k]).collect::<Vec<_>>();
            let lines = ins.iter().map(|i| i.lines.0).collect::<Vec<_>>();
            let want = (0..ins.len()).map(|k| s.writes[k] && rng.chance(20)).collect::<Vec<_>>();
            let mut r = Rng::new(rng.next());
            ins = insert(ins, &want, |k| {
                let line = lines[k];
                let mut block = Vec::new();
                if r.chance(60) && !off("arith") {
                    // b <= a * a, always
                    let a = r.range(2, 40);
                    let b = r.range(-60, a * a);
                    block.push(Ins::ad(op::LOADN, scratch, a as i16, line));
                    block.push(Ins::new(op::MUL, scratch, scratch, scratch, line));
                    block.push(Ins::ad(op::LOADN, scratch + 1, b as i16, line));
                    let mut test = Ins::local_jump(op::JUMPIFLE, scratch + 1, usize::MAX, line);
                    test.aux = Some(scratch as u32);
                    block.push(test);
                } else {
                    let truthy = r.chance(50);
                    block.push(Ins::new(op::LOADB, scratch, truthy as u8, 0, line));
                    block.push(Ins::local_jump(if truthy { op::JUMPIF } else { op::JUMPIFNOT }, scratch, usize::MAX, line));
                }
                block.extend(junk(&mut r, scratch, &consts, &originals, line));
                block
            });
        }
    }

    // blocks move last: where a scratch register may be written is worked out in execution
    // order, which is layout order only until they move
    if level >= 2 && !off("blocks") {
        ins = shuffle_blocks(ins, rng);
    }

    // lay it out, and carry along everything that names a pc
    let (code, lines, at) = encode(&ins)?;
    let mut moved = HashMap::new();
    for (j, i) in ins.iter().enumerate() {
        if let Some(o) = i.orig {
            moved.insert(o, at[j]);
        }
    }
    let new_pc = |old: usize| -> Result<u32, String> {
        if old == size {
            return Ok(code.len() as u32);
        }
        starts.get(&old).and_then(|k| moved.get(k)).map(|&pc| pc as u32).ok_or_else(|| format!("pc {old} isn't an instruction"))
    };
    if let Some(d) = &mut p.debug {
        if level >= 2 {
            d.locals.clear();
        } else {
            for l in &mut d.locals {
                l.start = new_pc(l.start as usize)?;
                l.end = new_pc(l.end as usize)?;
            }
        }
    }
    if let Some(t) = &mut p.types {
        if level >= 2 {
            t.locals.clear();
        } else {
            for l in &mut t.locals {
                let start = new_pc(l.start as usize)?;
                let end = new_pc((l.start + l.len) as usize)?;
                l.start = start;
                l.len = end.saturating_sub(start);
            }
        }
    }
    for fb in &mut p.feedback {
        if let Feedback::CallTarget(pc) = fb {
            *pc = new_pc(*pc as usize)?;
        }
    }
    if p.lines.is_some() {
        p.lines = Some(lines);
    }
    p.code = code;
    Ok(())
}

// Rewrites a chunk at a level from 1 to LEVELS. A function the rewrite can't handle stays as
// it was
pub fn apply(bytecode: &[u8], level: u8) -> Result<Vec<u8>, String> {
    apply_counted(bytecode, level).map(|(b, _, _)| b)
}

// Reads rewritten bytecode back and checks what the VM would trust blindly: every opcode is
// one Luau has, every jump lands on an instruction, and every function's code is followed to
// its end without falling off it
pub fn verify(bytecode: &[u8]) -> Result<(), String> {
    let chunk = Chunk::parse(bytecode)?;
    for (id, p) in chunk.protos.iter().enumerate() {
        let f = decode(p).map_err(|e| format!("function {id}: {e}"))?;
        for (k, i) in f.ins.iter().enumerate() {
            if i.op > 94 {
                return Err(format!("function {id}: opcode {} at instruction {k}", i.op));
            }
            if is_fastcall(i.op) && i.target.is_some_and(|t| f.ins[t].op != op::CALL) {
                return Err(format!("function {id}: the fast call at instruction {k} doesn't skip to a CALL"));
            }
            if i.op == op::NAMECALL && f.ins.get(k + 1).is_none_or(|c| c.op != op::CALL) {
                return Err(format!("function {id}: NAMECALL at {k} isn't followed by CALL"));
            }
        }
        if f.ins.last().is_some_and(|i| !ends_block(i.op)) {
            return Err(format!("function {id}: the last instruction falls off the end"));
        }
    }
    Ok(())
}

// apply, and how many of the chunk's functions it rewrote out of how many
pub fn apply_counted(bytecode: &[u8], level: u8) -> Result<(Vec<u8>, usize, usize), String> {
    let mut chunk = Chunk::parse(bytecode)?;
    let mut done = 0;
    // seeded by the program itself: the same bytes for every decompiler, every run
    let mut seed: u64 = 0xcbf2_9ce4_8422_2325 ^ level as u64;
    for &b in bytecode {
        seed = (seed ^ b as u64).wrapping_mul(0x100_0000_01b3);
    }
    let mut rng = Rng::new(seed);
    // ROSETTA_TRANSFORM_ONLY=a-b rewrites only those functions (to find which one breaks)
    let only = std::env::var("ROSETTA_TRANSFORM_ONLY").ok().and_then(|v| {
        let (a, b) = v.split_once('-').unwrap_or((&v, &v));
        Some(a.parse::<usize>().ok()?..=b.parse::<usize>().ok()?)
    });
    for (id, p) in chunk.protos.iter_mut().enumerate() {
        let mut copy = p.clone();
        let mut r = Rng::new(rng.next());
        if only.as_ref().is_some_and(|o| !o.contains(&id)) {
            continue;
        }
        match transform_proto(&mut copy, &mut r, level) {
            Ok(()) => {
                *p = copy;
                done += 1;
            }
            Err(e) => {
                if std::env::var_os("ROSETTA_TRACE_TRANSFORM").is_some() {
                    eprintln!("transform: a function stays as it was: {e}");
                }
            }
        }
    }
    if level >= 3 {
        if !off("protos") {
            shuffle_protos(&mut chunk, &mut rng);
        }
        for p in chunk.protos.iter_mut().filter(|_| !off("names")) {
            if let Some(d) = &mut p.debug {
                d.upvalues.clear();
                if d.locals.is_empty() {
                    p.debug = None;
                }
            }
        }
    }
    Ok((chunk.write(), done, chunk.protos.len()))
}

// a new order for the functions in the chunk, each still after the ones it makes closures of
fn shuffle_protos(chunk: &mut Chunk, rng: &mut Rng) {
    let n = chunk.protos.len();
    let mut parents = vec![Vec::new(); n];
    let mut pending = vec![0usize; n];
    for (i, p) in chunk.protos.iter().enumerate() {
        let mut kids = p.children.clone();
        for c in &p.constants {
            if let Constant::Closure(id) = c {
                kids.push(*id);
            }
        }
        kids.sort_unstable();
        kids.dedup();
        for &c in &kids {
            parents[c as usize].push(i);
        }
        pending[i] = kids.len();
    }
    let mut ready = (0..n).filter(|&i| pending[i] == 0).collect::<Vec<_>>();
    let mut order = Vec::with_capacity(n);
    while !ready.is_empty() {
        let i = ready.swap_remove(rng.below(ready.len()));
        order.push(i);
        for &p in &parents[i] {
            pending[p] -= 1;
            if pending[p] == 0 {
                ready.push(p);
            }
        }
    }
    if order.len() != n {
        return;
    }
    let mut new_id = vec![0u32; n];
    for (pos, &old) in order.iter().enumerate() {
        new_id[old] = pos as u32;
    }
    let mut protos = order.iter().map(|&i| chunk.protos[i].clone()).collect::<Vec<_>>();
    for p in &mut protos {
        for c in &mut p.children {
            *c = new_id[*c as usize];
        }
        for k in &mut p.constants {
            if let Constant::Closure(id) = k {
                *id = new_id[*id as usize];
            }
        }
    }
    chunk.protos = protos;
    chunk.main = new_id[chunk.main as usize];
}

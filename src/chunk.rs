// All of Luau's bytecode format, as Rosetta's Luau writes it (version 14, type encoding 3):
// read into plain structures and written back byte for byte, so the transform tier can change a
// program's bytecode and hand the result to a decompiler. Mirrors BytecodeBuilder::finalize and
// writeFunction; `rosetta transform-check` checks that writing what was read gives the same bytes.

pub const VERSION: u8 = 14;
pub const TYPES_VERSION: u8 = 3;
const LPF_INLINABLE: u8 = 1 << 3;

#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub version: u8,
    pub types_version: u8,
    pub strings: Vec<Vec<u8>>,
    // (type index + 1, name string), as the userdata type mapping lists them
    pub userdata: Vec<(u8, u32)>,
    pub protos: Vec<Proto>,
    pub main: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Proto {
    pub max_stack: u8,
    pub num_params: u8,
    pub num_upvalues: u8,
    pub is_vararg: u8,
    pub flags: u8,
    pub types: Option<Types>,
    // instruction words, AUX words included
    pub code: Vec<u32>,
    pub constants: Vec<Constant>,
    pub children: Vec<u32>,
    pub line_defined: u32,
    pub debug_name: u32,
    // the line of every word of code
    pub lines: Option<Vec<i32>>,
    pub debug: Option<Debug>,
    pub feedback: Vec<Feedback>,
    pub cost: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Types {
    pub function: Vec<u8>,
    pub upvalues: Vec<u8>,
    pub locals: Vec<TypedLocal>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedLocal {
    pub ty: u8,
    pub reg: u8,
    pub start: u32,
    pub len: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Debug {
    pub locals: Vec<LocalVar>,
    pub upvalues: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocalVar {
    pub name: u32,
    pub start: u32,
    pub end: u32,
    pub reg: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Feedback {
    CallTarget(u32),
    Construct(u32),
}

// numbers as bits, so a NaN's payload and -0 come back exactly
#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Nil,
    Boolean(u8),
    Number(u64),
    String(u32),
    Import(u32),
    Table(Vec<u32>),
    TableWithConstants(Vec<(u32, i32)>),
    Closure(u32),
    Vector([u32; 4]),
    VectorD([u64; 4]),
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u8(&mut self) -> Result<u8, String> {
        let v = *self.b.get(self.at).ok_or("unexpected end of bytecode")?;
        self.at += 1;
        Ok(v)
    }

    fn u32(&mut self) -> Result<u32, String> {
        let bytes = self.bytes(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn u64(&mut self) -> Result<u64, String> {
        let bytes = self.bytes(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn bytes(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.b.len()).ok_or("unexpected end of bytecode")?;
        let s = &self.b[self.at..end];
        self.at = end;
        Ok(s)
    }

    fn varint(&mut self) -> Result<u32, String> {
        let mut value = 0u32;
        for shift in (0..35).step_by(7) {
            let byte = self.u8()?;
            value |= ((byte & 0x7f) as u32) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err("varint too long".into())
    }
}

fn varint(out: &mut Vec<u8>, mut v: u32) {
    loop {
        let byte = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(byte);
            return;
        }
        out.push(byte | 0x80);
    }
}

// BytecodeBuilder's log2: the largest r with 2^r <= v
fn log2(v: usize) -> u32 {
    let mut r = 0;
    while v >= (2 << r) {
        r += 1;
    }
    r
}

impl Chunk {
    pub fn parse(bytes: &[u8]) -> Result<Chunk, String> {
        let mut r = Reader { b: bytes, at: 0 };
        let version = r.u8()?;
        if version != VERSION {
            return Err(format!("bytecode version {version}, only {VERSION} is supported"));
        }
        let types_version = r.u8()?;
        if types_version != TYPES_VERSION {
            return Err(format!("type encoding version {types_version}, only {TYPES_VERSION} is supported"));
        }
        let mut strings = Vec::new();
        for _ in 0..r.varint()? {
            let n = r.varint()? as usize;
            strings.push(r.bytes(n)?.to_vec());
        }
        let mut userdata = Vec::new();
        loop {
            let index = r.u8()?;
            if index == 0 {
                break;
            }
            userdata.push((index, r.varint()?));
        }
        let mut protos = Vec::new();
        for _ in 0..r.varint()? {
            let size = r.varint()? as usize;
            let data = r.bytes(size)?;
            let mut pr = Reader { b: data, at: 0 };
            protos.push(Proto::parse(&mut pr)?);
            if pr.at != data.len() {
                return Err(format!("function {} has {} bytes left over", protos.len() - 1, data.len() - pr.at));
            }
        }
        let main = r.varint()?;
        if r.at != bytes.len() {
            return Err("bytes after the main function's index".into());
        }
        Ok(Chunk { version, types_version, strings, userdata, protos, main })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut out = vec![self.version, self.types_version];
        varint(&mut out, self.strings.len() as u32);
        for s in &self.strings {
            varint(&mut out, s.len() as u32);
            out.extend_from_slice(s);
        }
        for &(index, name) in &self.userdata {
            out.push(index);
            varint(&mut out, name);
        }
        out.push(0);
        varint(&mut out, self.protos.len() as u32);
        for p in &self.protos {
            let data = p.write();
            varint(&mut out, data.len() as u32);
            out.extend_from_slice(&data);
        }
        varint(&mut out, self.main);
        out
    }
}

impl Proto {
    fn parse(r: &mut Reader) -> Result<Proto, String> {
        let max_stack = r.u8()?;
        let num_params = r.u8()?;
        let num_upvalues = r.u8()?;
        let is_vararg = r.u8()?;
        let flags = r.u8()?;
        let type_size = r.varint()? as usize;
        let types = if type_size == 0 {
            None
        } else {
            let mut t = Reader { b: r.bytes(type_size)?, at: 0 };
            let (f, u, l) = (t.varint()? as usize, t.varint()? as usize, t.varint()? as usize);
            let function = t.bytes(f)?.to_vec();
            let upvalues = t.bytes(u)?.to_vec();
            let mut locals = Vec::new();
            for _ in 0..l {
                locals.push(TypedLocal { ty: t.u8()?, reg: t.u8()?, start: t.varint()?, len: t.varint()? });
            }
            if t.at != type_size {
                return Err("type information has bytes left over".into());
            }
            Some(Types { function, upvalues, locals })
        };
        let size_code = r.varint()? as usize;
        let mut code = Vec::with_capacity(size_code);
        for _ in 0..size_code {
            code.push(r.u32()?);
        }
        let mut constants = Vec::new();
        for _ in 0..r.varint()? {
            constants.push(match r.u8()? {
                0 => Constant::Nil,
                1 => Constant::Boolean(r.u8()?),
                2 => Constant::Number(r.u64()?),
                3 => Constant::String(r.varint()?),
                4 => Constant::Import(r.u32()?),
                5 => {
                    let n = r.varint()?;
                    Constant::Table((0..n).map(|_| r.varint()).collect::<Result<_, _>>()?)
                }
                6 => Constant::Closure(r.varint()?),
                7 => Constant::Vector([r.u32()?, r.u32()?, r.u32()?, r.u32()?]),
                8 => {
                    let n = r.varint()?;
                    Constant::TableWithConstants((0..n).map(|_| Ok((r.varint()?, r.u32()? as i32))).collect::<Result<_, String>>()?)
                }
                11 => Constant::VectorD([r.u64()?, r.u64()?, r.u64()?, r.u64()?]),
                t => return Err(format!("constant type {t} isn't supported")),
            });
        }
        let children = (0..r.varint()?).map(|_| r.varint()).collect::<Result<_, _>>()?;
        let line_defined = r.varint()?;
        let debug_name = r.varint()?;
        let lines = if r.u8()? != 0 {
            let log = r.u8()? as u32;
            let mut lines = Vec::with_capacity(size_code);
            let mut last = 0u8;
            let mut offsets = Vec::with_capacity(size_code);
            for _ in 0..size_code {
                last = last.wrapping_add(r.u8()?);
                offsets.push(last);
            }
            let intervals = ((size_code.max(1) - 1) >> log) + 1;
            let mut base = Vec::with_capacity(intervals);
            let mut line = 0i32;
            for _ in 0..intervals {
                line = line.wrapping_add(r.u32()? as i32);
                base.push(line);
            }
            for (i, off) in offsets.iter().enumerate() {
                lines.push(base[i >> log] + *off as i32);
            }
            Some(lines)
        } else {
            None
        };
        let debug = if r.u8()? != 0 {
            let mut locals = Vec::new();
            for _ in 0..r.varint()? {
                locals.push(LocalVar { name: r.varint()?, start: r.varint()?, end: r.varint()?, reg: r.u8()? });
            }
            let upvalues = (0..r.varint()?).map(|_| r.varint()).collect::<Result<_, _>>()?;
            Some(Debug { locals, upvalues })
        } else {
            None
        };
        let mut feedback = Vec::new();
        for _ in 0..r.varint()? {
            feedback.push(match r.u8()? {
                0 => Feedback::CallTarget(r.varint()?),
                1 => Feedback::Construct(r.varint()?),
                k => return Err(format!("feedback slot kind {k} isn't supported")),
            });
        }
        let cost = if flags & LPF_INLINABLE != 0 { Some(r.varint()?) } else { None };
        Ok(Proto { max_stack, num_params, num_upvalues, is_vararg, flags, types, code, constants, children, line_defined, debug_name, lines, debug, feedback, cost })
    }

    fn write(&self) -> Vec<u8> {
        let mut out = vec![self.max_stack, self.num_params, self.num_upvalues, self.is_vararg, self.flags];
        match &self.types {
            None => varint(&mut out, 0),
            Some(t) => {
                let mut tb = Vec::new();
                varint(&mut tb, t.function.len() as u32);
                varint(&mut tb, t.upvalues.len() as u32);
                varint(&mut tb, t.locals.len() as u32);
                tb.extend_from_slice(&t.function);
                tb.extend_from_slice(&t.upvalues);
                for l in &t.locals {
                    tb.push(l.ty);
                    tb.push(l.reg);
                    varint(&mut tb, l.start);
                    varint(&mut tb, l.len);
                }
                varint(&mut out, tb.len() as u32);
                out.extend_from_slice(&tb);
            }
        }
        varint(&mut out, self.code.len() as u32);
        for w in &self.code {
            out.extend_from_slice(&w.to_le_bytes());
        }
        varint(&mut out, self.constants.len() as u32);
        for c in &self.constants {
            match c {
                Constant::Nil => out.push(0),
                Constant::Boolean(b) => out.extend_from_slice(&[1, *b]),
                Constant::Number(n) => {
                    out.push(2);
                    out.extend_from_slice(&n.to_le_bytes());
                }
                Constant::String(s) => {
                    out.push(3);
                    varint(&mut out, *s);
                }
                Constant::Import(id) => {
                    out.push(4);
                    out.extend_from_slice(&id.to_le_bytes());
                }
                Constant::Table(keys) => {
                    out.push(5);
                    varint(&mut out, keys.len() as u32);
                    keys.iter().for_each(|&k| varint(&mut out, k));
                }
                Constant::Closure(p) => {
                    out.push(6);
                    varint(&mut out, *p);
                }
                Constant::Vector(v) => {
                    out.push(7);
                    v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
                }
                Constant::TableWithConstants(entries) => {
                    out.push(8);
                    varint(&mut out, entries.len() as u32);
                    for &(k, c) in entries {
                        varint(&mut out, k);
                        out.extend_from_slice(&c.to_le_bytes());
                    }
                }
                Constant::VectorD(v) => {
                    out.push(11);
                    v.iter().for_each(|x| out.extend_from_slice(&x.to_le_bytes()));
                }
            }
        }
        varint(&mut out, self.children.len() as u32);
        self.children.iter().for_each(|&c| varint(&mut out, c));
        varint(&mut out, self.line_defined);
        varint(&mut out, self.debug_name);
        match &self.lines {
            Some(lines) if !lines.is_empty() => {
                out.push(1);
                write_lines(&mut out, lines);
            }
            _ => out.push(0),
        }
        match &self.debug {
            Some(d) => {
                out.push(1);
                varint(&mut out, d.locals.len() as u32);
                for l in &d.locals {
                    varint(&mut out, l.name);
                    varint(&mut out, l.start);
                    varint(&mut out, l.end);
                    out.push(l.reg);
                }
                varint(&mut out, d.upvalues.len() as u32);
                d.upvalues.iter().for_each(|&u| varint(&mut out, u));
            }
            None => out.push(0),
        }
        varint(&mut out, self.feedback.len() as u32);
        for f in &self.feedback {
            match f {
                Feedback::CallTarget(pc) => {
                    out.push(0);
                    varint(&mut out, *pc);
                }
                Feedback::Construct(shape) => {
                    out.push(1);
                    varint(&mut out, *shape);
                }
            }
        }
        if self.flags & LPF_INLINABLE != 0 {
            varint(&mut out, self.cost.unwrap_or(0));
        }
        out
    }
}

// BytecodeBuilder::writeLineInfo: each word's line as an 8-bit delta from its span's baseline,
// the span the largest power of two that keeps every delta in 8 bits
fn write_lines(out: &mut Vec<u8>, lines: &[i32]) {
    let n = lines.len();
    let mut span: usize = 1 << 24;
    let mut offset = 0;
    while offset < n {
        let mut next = offset;
        let (mut min, mut max) = (lines[offset], lines[offset]);
        while next < n && next < offset + span {
            min = min.min(lines[next]);
            max = max.max(lines[next]);
            if max - min > 255 {
                break;
            }
            next += 1;
        }
        if next < n && next - offset < span {
            span = 1 << log2(next - offset);
        }
        offset += span;
    }
    let log = log2(span);
    let baselines = (0..(n - 1) / span + 1).map(|k| lines[k * span..((k + 1) * span).min(n)].iter().copied().min().unwrap()).collect::<Vec<_>>();
    out.push(log as u8);
    let mut last = 0u8;
    for (i, &line) in lines.iter().enumerate() {
        let delta = (line - baselines[i >> log]) as u8;
        out.push(delta.wrapping_sub(last));
        last = delta;
    }
    let mut last = 0i32;
    for &b in &baselines {
        out.extend_from_slice(&(b.wrapping_sub(last)).to_le_bytes());
        last = b;
    }
}

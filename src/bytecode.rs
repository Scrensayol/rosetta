// Just enough of Luau's bytecode format to read each function's instructions, for the mutation
// fuzzer's novelty signal. It reads bytecode version 12 and later, where every function is
// prefixed by its size, so everything after the code can be skipped without parsing it.

use std::collections::HashSet;

const JUMP: u8 = 23;
const JUMPBACK: u8 = 24;
const JUMPX: u8 = 67;

// opcodes followed by an AUX word (Luau's getOpLength)
fn has_aux(op: u8) -> bool {
    matches!(op, 7 | 8 | 12 | 15 | 16 | 20 | 27..=32 | 53 | 55 | 58 | 60 | 66 | 74 | 75 | 77..=80 | 83..=88 | 90 | 91 | 93 | 94)
}

// opcodes whose D is a jump offset
fn jumps(op: u8) -> bool {
    matches!(op, JUMP | JUMPBACK | 25..=32 | 56..=59 | 61 | 76 | 77..=80)
}

struct Reader<'a> {
    b: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn u8(&mut self) -> Option<u8> {
        let v = *self.b.get(self.at)?;
        self.at += 1;
        Some(v)
    }

    fn varint(&mut self) -> Option<usize> {
        let mut value = 0usize;
        for shift in (0..35).step_by(7) {
            let byte = self.u8()?;
            value |= ((byte & 0x7f) as usize) << shift;
            if byte & 0x80 == 0 {
                return Some(value);
            }
        }
        None
    }

    fn skip(&mut self, n: usize) -> Option<()> {
        self.at = self.at.checked_add(n).filter(|&e| e <= self.b.len())?;
        Some(())
    }

    fn u32(&mut self) -> Option<u32> {
        let bytes = self.b.get(self.at..self.at + 4)?;
        self.at += 4;
        Some(u32::from_le_bytes(bytes.try_into().unwrap()))
    }
}

// Every function's instructions, AUX words dropped, each with its jump target's index when it
// jumps. None when the bytecode isn't a format this reads
pub fn functions(bytecode: &[u8]) -> Option<Vec<Vec<(u8, Option<usize>)>>> {
    let mut r = Reader { b: bytecode, at: 0 };
    let version = r.u8()?;
    if version < 12 {
        return None;
    }
    let types = r.u8()?;
    for _ in 0..r.varint()? {
        let len = r.varint()?;
        r.skip(len)?;
    }
    if types == 3 {
        while r.u8()? != 0 {
            r.varint()?;
        }
    }
    let mut out = Vec::new();
    for _ in 0..r.varint()? {
        let size = r.varint()?;
        let end = r.at.checked_add(size)?;
        r.skip(5)?;
        let typesize = r.varint()?;
        r.skip(typesize)?;
        let code = (0..r.varint()?).map(|_| r.u32()).collect::<Option<Vec<_>>>()?;
        // pc -> instruction index, for jump targets
        let mut index = vec![usize::MAX; code.len() + 1];
        let mut insns = Vec::new();
        let mut pc = 0;
        while pc < code.len() {
            index[pc] = insns.len();
            let op = (code[pc] & 0xff) as u8;
            let target = if jumps(op) {
                Some(pc as i64 + 1 + ((code[pc] as i32) >> 16) as i64)
            } else if op == JUMPX {
                Some(pc as i64 + 1 + ((code[pc] as i32) >> 8) as i64)
            } else {
                None
            };
            insns.push((op, target));
            pc += if has_aux(op) { 2 } else { 1 };
        }
        index[code.len()] = insns.len();
        let insns = insns
            .into_iter()
            .map(|(op, target)| (op, target.and_then(|t| usize::try_from(t).ok()).and_then(|t| index.get(t).copied()).filter(|&i| i != usize::MAX)))
            .collect();
        out.push(insns);
        if end > bytecode.len() {
            return None;
        }
        r.at = end;
    }
    Some(out)
}

// What a decompiler sees, reduced to small numbers: opcode pairs and triples in each function,
// and every jump's direction and landing opcode. A program is new to the fuzzer when it has one
// of these that nothing before it had. tag keeps different compile settings apart
pub fn features(bytecode: &[u8], tag: u32, out: &mut HashSet<u32>) {
    let Some(functions) = functions(bytecode) else { return };
    let mix = |kind: u32, a: u32, b: u32, c: u32| (tag << 28) ^ (kind << 24) ^ (a << 16) ^ (b << 8) ^ c;
    for f in &functions {
        for (i, &(op, target)) in f.iter().enumerate() {
            if let Some(&(next, _)) = f.get(i + 1) {
                out.insert(mix(1, op as u32, next as u32, 0));
                if let Some(&(third, _)) = f.get(i + 2) {
                    out.insert(mix(2, op as u32, next as u32, third as u32));
                }
            }
            if let Some(t) = target {
                let landing = f.get(t).map_or(255, |x| x.0) as u32;
                out.insert(mix(3, op as u32, landing, (t <= i) as u32));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::luau::{self, Profile};

    #[test]
    fn reads_compiled_functions() {
        let src = "local function f(a) for i = 1, a do if i % 2 == 0 then print(i) end end end\nf(3)\nlocal t = {x = 1}\nprint(t.x)";
        for profile in Profile::ALL {
            let bytecode = luau::compile(src, profile).unwrap();
            let functions = functions(&bytecode).expect("parses");
            assert_eq!(functions.len(), 2, "{profile}");
            assert!(functions.iter().flatten().any(|&(op, t)| op == 57 && t.is_some()), "FORNLOOP with a target at {profile}");
            assert!(functions.iter().all(|f| f.last().is_some_and(|&(op, _)| op == 22)), "ends in RETURN at {profile}");
        }
    }
}

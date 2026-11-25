-- evaluation order a decompiler can't move: locals used once whose calls have side effects,
-- the operands of > and >= (compiled as < and <= with the operands swapped), a method call's
-- object and arguments, and values read before a later write or call changes them
local trace = {}
local function note(tag, value)
	table.insert(trace, tag)
	return value
end
local function flush(label)
	print(label, table.concat(trace, " "))
	table.clear(trace)
end

-- two locals used once each, in the opposite order: inlining both would swap the calls
local first = note("a", 1)
local second = note("b", 2)
print(second, first)
flush("locals")

-- > and >= compile to LT and LE with the operands swapped, but they still run left to right
print(note("l", 3) > note("r", 2), note("l", 1) >= note("r", 1), note("l", "x") < note("r", "y"))
flush("compare")
print(not (note("l", 1) <= note("r", 2)), note("l", 2) ~= note("r", 2))
flush("compare not")
if note("cond l", 5) > note("cond r", 4) then
	note("then", nil)
end
flush("branch")

-- the object of a method call is worked out once, before its arguments
local obj = { n = 0 }
function obj:add(k)
	self.n += k
	return self
end
local function getobj()
	note("obj", nil)
	return obj
end
getobj():add(note("arg", 1)):add(note("arg2", 2))
print(obj.n)
flush("method")

-- a value read into a local before the table changes keeps the old value
local t = { v = "old" }
local saved = t.v
t.v = "new"
print(saved, t.v)
local box = { 1, 2, 3 }
local len = #box
table.insert(box, 4)
print(len, #box)

-- a call that changes a local the same argument list reads before and after it
local counter = 0
local function bump()
	counter += 1
	return counter
end
print(counter, bump(), counter + 0, bump())
local before = counter
bump()
print(before, counter)

-- an index read before a call in the same expression moves it
local arr = { 10, 20, 30 }
local i = 1
local function advance()
	i += 1
	return 0
end
print(arr[i] + advance(), arr[i])

-- every operand runs left to right before anything combines, right-associative or not
print(note("1", "a") .. note("2", "b") .. note("3", "c"))
flush("concat")
print(note("x", 2) ^ note("y", 3) ^ note("z", 1), note("p", 10) - note("q", 3) - note("r", 2))
flush("arith")

-- and / or only run what they need
local hit = note("left", false) and note("never", true) or note("right", "ok")
print(hit)
flush("logic")

-- a constructor evaluates its entries in order, keyed or not
local made = { note("one", 1), k = note("key", 2), note("two", 3), [note("ikey", "x")] = note("ival", 4) }
print(made[1], made[2], made.k, made.x)
flush("constructor")

-- an assignment reads its target table before the right-hand side runs, so a call that replaces
-- the table writes into the old one
local holder = { inner = { v = 0 } }
local function swap()
	note("swap", nil)
	local old = holder.inner
	holder.inner = { v = 100 }
	return old.v + 1
end
holder.inner.v = swap()
print(holder.inner.v)
flush("target")

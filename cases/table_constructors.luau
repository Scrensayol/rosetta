-- table constructors: positional entries overwrite earlier [1] =, a call or ... last expands but
-- anywhere else gives one value, nil holes, and keys that need brackets
local function two()
	return "a", "b"
end
local function show(t, n)
	local out = {}
	for i = 1, n do
		out[i] = tostring(t[i])
	end
	return table.concat(out, ",")
end

print(show({ [1] = "x", "y" }, 2))
print(show({ "y", [1] = "x" }, 2))
print(show({ two() }, 3), show({ two(), "c" }, 3), show({ (two()) }, 3), show({ two(), two() }, 4))
print(show({ nil, 2, nil, 4 }, 4), select("#", table.unpack({ 1, nil, 3 }, 1, 3)))

local function pack(...)
	return { ... }, { n = select("#", ...), ... }, { ..., "last" }
end
local a, b, c = pack(1, nil, 3)
print(show(a, 3), b.n, show(b, 3), show(c, 3))

local k = "dyn"
local t = {
	plain = 1,
	["with space"] = 2,
	["end"] = 3,
	[k] = 4,
	[k .. "2"] = 5,
	[10] = 6,
	[-1] = 7,
	[1.5] = 8,
	[true] = 9,
	nested = { deep = { deeper = { 10 } } },
	fn = function(self)
		return self.plain
	end,
	"pos1",
	"pos2",
}
print(t.plain, t["with space"], t["end"], t.dyn, t.dyn2, t[10], t[-1], t[1.5], t[true])
print(t.nested.deep.deeper[1], t:fn(), t[1], t[2], #t)

-- the same key twice: the later one wins
local dup = { x = 1, x = 2, [2] = "a", "b", "c" }
print(dup.x, dup[1], dup[2])

-- assignment chains through fresh tables
local root = {}
root.a = {}
root.a.b = {}
root.a.b.c = "set"
root["a"]["b"].d = root.a.b.c .. "!"
print(root.a.b.c, root.a.b.d)

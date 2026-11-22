-- multiple assignment and multiple returns: every right-hand side is read before anything is
-- assigned, a call in the middle of a list is cut to one value, and parentheses cut it too
local function three()
	return 1, 2, 3
end
local function none() end

local a, b, c = 1, 2, 3
a, b, c = c, a, b
print(a, b, c)

local t = { 10, 20, 30 }
local i = 1
i, t[i] = i + 1, "x" -- the index is read before i changes
print(i, t[1], t[2])

local x, y, z = three(), 10
print(x, y, z)
print((three()))
print(three(), three())
print(#{ three(), three() }, #{ (three()), three() }, select("#", none()), select("#", (none())))

local function count(...)
	return select("#", ...)
end
print(count(nil, nil), count(none()), count(none(), nil), count(three()))

local function tail(...)
	local first, rest = ..., select(2, ...)
	return first, rest, select(-1, ...)
end
print(tail(4, 5, 6))

-- a swap through a table and a local in one statement
local cell = { v = 1 }
local held = 2
cell.v, held = held, cell.v
print(cell.v, held)

-- varargs captured into a table with holes
local function pack(...)
	return { n = select("#", ...), ... }
end
local p = pack(1, nil, 3, nil)
print(p.n, p[1], p[2], p[3], p[4])

-- assignment to a local that a later expression in the same list reads
local m = 1
local n
m, n = m + 1, m
print(m, n)

-- compound assignment whose index expression has a side effect, evaluated once
local calls = 0
local function idx()
	calls += 1
	return 2
end
local arr = { 1, 2, 3 }
arr[idx()] += 10
arr[idx()] ..= ""
print(arr[2], calls)

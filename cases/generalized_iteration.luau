-- generic for without an iterator function: plain tables (array part, then hash part), __iter
-- returning a closure or next, objects with __call, coroutines, and pairs and ipairs, which the
-- compiler turns into their own loop instructions
local function keys(t)
	local out = {}
	for k in t do
		table.insert(out, tostring(k))
	end
	table.sort(out)
	return table.concat(out, ",")
end

local list = { "a", "b", "c" }
for i, v in list do
	print("plain", i, v)
end
for i, v in ipairs(list) do
	print("ipairs", i, v)
end
for i, v in pairs(list) do
	print("pairs", i, v)
end
print(keys({ x = 1, y = 2, [3] = true, [true] = 0 }))

-- a hole: ipairs stops at it, a plain loop and pairs walk past it
local holes = { 1, 2, nil, 4 }
local viaIpairs, viaPlain, viaPairs = 0, 0, 0
for _ in ipairs(holes) do
	viaIpairs += 1
end
for _ in holes do
	viaPlain += 1
end
for _ in pairs(holes) do
	viaPairs += 1
end
print(viaIpairs, viaPlain, viaPairs)

-- __iter returning a stateful closure
local Range = {}
Range.__index = Range
Range.__iter = function(self)
	local i = self.from - self.step
	return function()
		i += self.step
		if i <= self.to then
			return i, i * i
		end
		return nil
	end
end
local function range(from, to, step)
	return setmetatable({ from = from, to = to, step = step or 1 }, Range)
end
for v, sq in range(1, 7, 3) do
	print("range", v, sq)
end

-- __iter handing back next and another table to walk
local Bag = {
	__iter = function(self)
		return next, self.items
	end,
}
local bag = setmetatable({ items = { "x", "y" }, hidden = true }, Bag)
for k, v in bag do
	print("bag", k, v)
end

-- an object with __call is the iterator function itself
local countdown = setmetatable({ n = 3 }, {
	__call = function(self)
		if self.n > 0 then
			self.n -= 1
			return self.n + 1
		end
		return nil
	end,
})
for v in countdown do
	print("countdown", v)
end

-- a coroutine as the iterator
for v in coroutine.wrap(function()
	for i = 1, 3 do
		coroutine.yield(i * 2)
	end
end) do
	print("co", v)
end

-- continue, break and writing the loop variable inside generalized loops
for i, v in list do
	if v == "b" then
		continue
	end
	i *= 10
	print("skip b", i, v)
end
local found
for i, v in { 5, 6, 7, 8 } do
	if v % 2 == 1 and i > 1 then
		found = i
		break
	end
end
print("found", found)

-- nested loops over the same table, the inner one reading the outer's variables
local grid = { { 1, 2 }, { 3, 4 } }
local sum = 0
for r, row in grid do
	for c, cell in row do
		sum += cell * r * c
	end
end
print("grid", sum)

-- a table changed between loops is walked as it is now
local growing = { 1 }
for round = 1, 3 do
	local seen = 0
	for _ in growing do
		seen += 1
	end
	growing[#growing + 1] = round
	print("round", round, seen)
end

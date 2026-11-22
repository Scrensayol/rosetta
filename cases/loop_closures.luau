-- closures made in loops: each iteration of a for loop has its own variable, a while loop's
-- local is fresh each time round, and an upvalue written after capture is shared
local fns = {}
for i = 1, 3 do
	fns[#fns + 1] = function()
		return i
	end
end
local k = 0
while k < 3 do
	k += 1
	local j = k * 10
	fns[#fns + 1] = function()
		return j
	end
end
for _, v in ipairs({ "a", "b" }) do
	local copy = v
	fns[#fns + 1] = function()
		copy = copy .. "!"
		return copy
	end
end
for idx, f in ipairs(fns) do
	print(idx, f(), f())
end

-- one variable captured by two closures, then changed from outside
local function counter()
	local n = 0
	local function inc()
		n += 1
		return n
	end
	local function get()
		return n
	end
	n = 100
	return inc, get
end
local inc, get = counter()
inc()
print(get(), inc(), get())

-- a loop variable shadowed inside the body, and the outer one read after
local total = 0
for i = 1, 4 do
	local i = i * 2
	total += i
end
local i = "outer"
print(total, i)

-- the loop body's local captured, then the loop exits early
local saved
for n = 1, 10 do
	local sq = n * n
	if sq > 20 then
		saved = function()
			return sq
		end
		break
	end
end
print(saved())

-- repeat whose condition reads a local from the body, captured by a closure there
local seen = {}
local r = 0
repeat
	r += 1
	local stop = r >= 3
	seen[r] = function()
		return stop
	end
until stop
print(r, seen[1](), seen[3]())

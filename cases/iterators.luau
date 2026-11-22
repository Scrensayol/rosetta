-- for loops: generic for over custom iterators (stateless, stateful, next directly), numeric for
-- with float and negative steps, bounds evaluated once, and a body that writes its loop variable
local function range(n)
	local i = 0
	return function()
		i += 1
		if i <= n then
			return i, i * i
		end
	end
end
for i, sq in range(3) do
	print("range", i, sq)
end

local function evens(t, i)
	i += 2
	if t[i] ~= nil then
		return i, t[i]
	end
end
local list = { "a", "b", "c", "d", "e" }
for i, v in evens, list, -1 do
	print("evens", i, v)
end

local sorted = { 10, 20, 30 }
for i, v in next, sorted do
	print("next", i, v)
end
for i, v in ipairs({ 1, 2, nil, 4 }) do
	print("ipairs", i, v)
end

for i = 1, 2, 0.5 do
	print("float", i)
end
for i = 3, 1, -1 do
	print("down", i)
end
for i = 1, 0 do
	print("never", i)
end
for i = 1.0, 3 do
	print("float start", i)
end

local calls = 0
local function bound(v)
	calls += 1
	return v
end
for i = bound(1), bound(3), bound(1) do
	print("bounded", i)
end
print("bound calls", calls)

-- writing the loop variable changes only this iteration's copy
for i = 1, 3 do
	local before = i
	i = i * 10
	print("copy", before, i)
end

-- a while loop with two exits and a repeat that runs once
local n, steps = 27, 0
while true do
	if n == 1 then
		break
	end
	if steps > 200 then
		print("too long")
		break
	end
	n = if n % 2 == 0 then n // 2 else 3 * n + 1
	steps += 1
end
print("collatz", steps)
repeat
	steps -= 1
until true
print("after repeat", steps)

-- nested loops where the inner break and continue only touch the inner loop
local found = {}
for i = 1, 4 do
	for j = 1, 4 do
		if j > i then
			break
		end
		if (i + j) % 2 == 1 then
			continue
		end
		found[#found + 1] = i * 10 + j
	end
end
print(table.concat(found, " "))

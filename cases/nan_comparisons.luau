-- comparisons a decompiler can't flip: with NaN, `not (a < b)` isn't `a >= b`, and `x ~= x` is
-- true. Also and/or chains where a false or nil value in the middle changes which side wins
local nan = 0 / 0
local values = { 1, -1, 0, nan, math.huge, -math.huge }

local function compare(a, b)
	local out = {}
	if not (a < b) then out[#out + 1] = "!lt" end
	if not (a <= b) then out[#out + 1] = "!le" end
	if not (a > b) then out[#out + 1] = "!gt" end
	if not (a >= b) then out[#out + 1] = "!ge" end
	if a ~= a then out[#out + 1] = "self" end
	if not (a == b) then out[#out + 1] = "!eq" end
	while not (a < b) do
		out[#out + 1] = "w"
		break
	end
	local rounds = 0
	repeat
		rounds += 1
		out[#out + 1] = "r"
	until not (a > b) or rounds == 2
	return table.concat(out, ",")
end

for i = 1, #values do
	for j = 1, #values do
		print(i, j, compare(values[i], values[j]))
	end
end

-- values, not just truthiness
local function pick(a, b, c)
	local x = a and b or c
	local y = a and (b or c)
	local z = (a or b) and c
	local w = not a == not b
	local v = a and false or c
	local u = a == nil and b or nil
	return x, y, z, w, v, u
end
local samples = { true, false, nil, 0, "", nan }
for i = 1, 6 do
	for j = 1, 6 do
		for k = 1, 6 do
			local a, b, c = samples[i], samples[j], samples[k]
			print(i, j, k, pick(a, b, c))
		end
	end
end

-- a guard whose branches both fall through, around a comparison with a constant on the left
local count = 0
for _, v in ipairs(values) do
	if 0 < v then
		count += 1
	elseif not (0 >= v) then
		count += 100
	end
end
print("count", count)

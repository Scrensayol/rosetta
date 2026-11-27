-- work that never shows up in print: globals written and removed, a library table extended,
-- tables changed through metamethods and table.insert/remove, coroutines resumed for their side
-- effects, and what the chunk returns. Only what's left behind tells a right decompile from a
-- wrong one, so the result has to be graded on that too
registry = {}
local log = {}

local function track(t, name)
	return setmetatable({}, {
		__index = t,
		__newindex = function(_, k, v)
			table.insert(log, name .. "." .. tostring(k))
			rawset(t, k, v)
		end,
	})
end

local config = track({}, "config")
config.speed = 16
config.jump = 50
config.speed += 4

for i = 1, 5 do
	if i % 2 == 0 then
		table.insert(registry, i * i)
	elseif i == 5 then
		registry.last = i
	end
end
table.remove(registry, 1)

function string.shout(s)
	return s:upper() .. "!"
end

counter = 0
local worker = coroutine.wrap(function()
	for _ = 1, 3 do
		counter += 1
		coroutine.yield()
	end
end)
worker()
worker()

temporary = true
temporary = nil

local swapped = { a = 1, b = 2 }
swapped.a, swapped.b = swapped.b, swapped.a
settings = { config = config, swapped = swapped, log = log }

print("done")
return #log, registry.last, ("hi"):shout()

-- getfenv and setfenv: once a script touches environments, the compiler stops assuming globals
-- are what they look like, so builtins aren't folded, fast calls check the environment and
-- imports are looked up again. A decompiler has to keep every global a global
local function area(r)
	return math.floor(math.pi * r * r)
end
print(area(2))

local fake = {
	math = {
		pi = 3,
		floor = function(x)
			return "floor " .. x
		end,
	},
}
setfenv(area, setmetatable(fake, { __index = getfenv() }))
print(area(2))
print(getfenv(area) == fake, getfenv(1) == getfenv(print))

-- a global written through one environment is invisible from another
local function writer()
	shared_value = "written"
	return shared_value
end
setfenv(writer, setmetatable({}, { __index = getfenv() }))
print(writer(), rawget(getfenv(writer), "shared_value"), shared_value)

-- setfenv(1, ...) swaps the running function's own environment
local function sandboxed()
	setfenv(1, setmetatable({ tag = "inside" }, { __index = getfenv(2) }))
	return tag, type(print), type(math.floor)
end
print(sandboxed())
print(tag)

-- a function's environment is shared with the closures it makes
local function maker()
	return function()
		return label
	end
end
setfenv(maker, setmetatable({ label = "from maker" }, { __index = getfenv() }))
print(maker()())

-- globals read in a loop are looked up every time, not once
local calls = 0
local env = setmetatable({}, {
	__index = function(_, key)
		calls += 1
		return getfenv()[key]
	end,
})
local function loop()
	local s = 0
	for i = 1, 3 do
		s += math.abs(-i)
	end
	return s
end
setfenv(loop, env)
print(loop(), calls)

-- builtins that the compiler would fold or fast-call
print(math.max(1, 5), math.sqrt(16), bit32.band(12, 10), string.len("four"), #tostring(12.5))

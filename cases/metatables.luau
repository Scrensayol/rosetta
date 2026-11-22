-- metamethods: arithmetic, comparison, concat, call, index and newindex functions, len and unm,
-- and the raw functions that skip them
local Vec = {}
Vec.__index = Vec
local function vec(x, y)
	return setmetatable({ x = x, y = y }, Vec)
end
Vec.__add = function(a, b)
	return vec(a.x + b.x, a.y + b.y)
end
Vec.__sub = function(a, b)
	return vec(a.x - b.x, a.y - b.y)
end
Vec.__mul = function(a, b)
	if type(a) == "number" then
		return vec(a * b.x, a * b.y)
	end
	return vec(a.x * b, a.y * b)
end
Vec.__unm = function(a)
	return vec(-a.x, -a.y)
end
Vec.__eq = function(a, b)
	return a.x == b.x and a.y == b.y
end
Vec.__lt = function(a, b)
	return a.x * a.x + a.y * a.y < b.x * b.x + b.y * b.y
end
Vec.__le = function(a, b)
	return not (b < a)
end
Vec.__len = function(a)
	return math.sqrt(a.x * a.x + a.y * a.y)
end
Vec.__concat = function(a, b)
	local function s(v)
		return if type(v) == "table" then `({v.x}, {v.y})` else tostring(v)
	end
	return s(a) .. s(b)
end
Vec.__call = function(self, k)
	return self[k]
end
function Vec:dot(o)
	return self.x * o.x + self.y * o.y
end

local a, b = vec(1, 2), vec(3, 4)
local c = a + b
print(c.x, c.y, (a - b).x, (2 * a).y, (a * 3).x, (-a).x)
print(a == vec(1, 2), a ~= b, a < b, a <= b, a > b, b >= a, rawequal(a, vec(1, 2)))
print(#b, a .. b, a .. "!", "v=" .. a, a("y"), a:dot(b), Vec.dot(a, b))

-- __index and __newindex as functions, logging every access
local log = {}
local store = {}
local proxy = setmetatable({}, {
	__index = function(_, k)
		log[#log + 1] = "get " .. tostring(k)
		return store[k]
	end,
	__newindex = function(_, k, v)
		log[#log + 1] = "set " .. tostring(k)
		store[k] = v
	end,
})
proxy.a = 1
proxy.a += 1
proxy[1] = proxy.a * 10
print(proxy.a, proxy[1], rawget(proxy, "a"), table.concat(log, ","))
rawset(proxy, "raw", true)
print(proxy.raw, #log)

-- inheritance through __index chains
local Base = {}
Base.__index = Base
function Base.new(name)
	return setmetatable({ name = name }, Base)
end
function Base:hello()
	return "base " .. self.name
end
local Derived = setmetatable({}, { __index = Base })
Derived.__index = Derived
function Derived.new(name)
	local self = Base.new(name)
	return setmetatable(self, Derived)
end
function Derived:hello()
	return "derived " .. Base.hello(self)
end
local d = Derived.new("d")
print(d:hello(), getmetatable(d) == Derived, Base.hello(d))

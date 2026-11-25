--!rosetta-options +t1
-- type syntax, which compiles to nothing, compiled with type information on (+t1): every
-- function carries its parameter types in the bytecode, which a decompiler has to read past,
-- and casts, generics, packs and attributes all have to come back as valid Luau
type Vec = { x: number, y: number }
export type Shape = { kind: "circle", r: number } | { kind: "rect", w: number, h: number }
type Callback<T> = (T, ...any) -> boolean
type Pair<K, V> = { key: K, value: V }

local function add(a: Vec, b: Vec): Vec
	return { x = a.x + b.x, y = a.y + b.y }
end

local function area(s: Shape): number
	if s.kind == "circle" then
		return math.floor(3 * s.r * s.r)
	end
	return (s :: any).w * (s :: any).h
end

local function first<T>(list: { T }, pred: Callback<T>): T?
	for _, v in list do
		if pred(v) then
			return v
		end
	end
	return nil
end

local function count(...: number): (number, number)
	local total = 0
	for i = 1, select("#", ...) do
		total += (select(i, ...))
	end
	return select("#", ...), total
end

local v = add({ x = 1, y = 2 }, { x = 3, y = 4 })
print(v.x, v.y)
print(area({ kind = "circle", r = 2 }), area({ kind = "rect", w = 3, h = 4 }))
print(first({ 1, 5, 8 }, function(n: number): boolean
	return n > 4
end))
print(count(1, 2, 3))

-- every primitive the type information can name, as parameters and typed locals
local function describe(n: number, s: string, t: { any }, f: () -> (), b: buffer, flag: boolean?, vec: vector, th: thread): string
	local size: number = buffer.len(b)
	local parts: { string } = { tostring(n), s, tostring(#t), type(f), tostring(size), tostring(flag), tostring(vec.y), type(th) }
	return table.concat(parts, " ")
end
local y = v.x // 2
print(describe(1.5, "s", { 1, 2 }, function() end, buffer.create(4), nil, vector.create(1, y, 3), coroutine.create(print)))

-- a cast keeps the call cut to one value
local function many(): ...number
	return 1, 2, 3
end
print((many() :: number), many())
local text = ("x" :: string):rep(3)
print(text, (#text :: number) + 1)

-- methods with an annotated self, and a type taken from a value
type Account = { balance: number, deposit: (Account, number) -> Account }
local Account = {}
Account.__index = Account
function Account.new(balance: number): Account
	return setmetatable({ balance = balance }, Account) :: any
end
function Account.deposit(self: Account, amount: number): Account
	self.balance += amount
	return self
end
local acct = Account.new(10)
type AccountValue = typeof(acct)
local same: AccountValue = acct:deposit(5):deposit(2.5)
print(same.balance)

-- an attribute: native functions keep their type information at any level
local scale = @native function(x: number, k: number): number
	return x * k
end
@native
local function lerp(a: number, b: number, t: number): number
	return a + (b - a) * t
end
print(scale(3, 4), lerp(0, 10, 0.25))

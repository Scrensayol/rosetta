-- The part of busted's API the library specs use, put in front of every spec.luau. As in
-- busted, the spec's top level only declares; a describe's body runs when the run reaches it,
-- after the tests before it (specs lean on that: a body can read what an earlier test's
-- before_each left behind). Each test runs after its describes' before_each hooks and prints one
-- line. Everything here is a local of the test chunk, so the library never sees any of it
local busted_root = { befores = {}, afters = {}, tests = {} }
local busted_current = busted_root

local function describe(name, body)
	table.insert(busted_current.tests, { describe = name, body = body })
end

local function it(name, body)
	table.insert(busted_current.tests, { name = name, body = body })
end

local function before_each(body)
	table.insert(busted_current.befores, body)
end

local function after_each(body)
	table.insert(busted_current.afters, body)
end

local busted_finally = nil
local function finally(body)
	table.insert(busted_finally, body)
end

local function busted_show(v)
	local kind = type(v)
	if kind == "string" then
		return string.format("%q", v)
	elseif kind == "number" or kind == "boolean" or kind == "nil" then
		return tostring(v)
	end
	-- tables and functions print by kind: their addresses say nothing, and a library's
	-- __tostring is code under test, which only the assertions get to judge
	return kind
end

local function busted_fail(message)
	error(message, 0)
end

local busted_count = 0

local function busted_same(a, b, seen)
	if a == b then
		return true
	end
	if type(a) ~= "table" or type(b) ~= "table" then
		return false
	end
	seen = seen or {}
	if seen[a] == b then
		return true
	end
	seen[a] = b
	for k, v in next, a do
		if not busted_same(v, rawget(b, k), seen) then
			return false
		end
	end
	for k in next, b do
		if rawget(a, k) == nil then
			return false
		end
	end
	return true
end

-- callable like luassert, so specs with their own runner keep using plain assert(...)
local assert = setmetatable({}, { __call = function(_, ...) return assert(...) end })
assert.are = assert
assert.is = assert

function assert.equal(expected, actual)
	busted_count += 1
	if expected ~= actual then
		busted_fail(`expected {busted_show(expected)}, got {busted_show(actual)}`)
	end
end
assert.equals = assert.equal

function assert.same(expected, actual)
	busted_count += 1
	if not busted_same(expected, actual) then
		busted_fail("tables differ")
	end
end

function assert.is_true(v)
	busted_count += 1
	if v ~= true then
		busted_fail(`expected true, got {busted_show(v)}`)
	end
end

function assert.is_false(v)
	busted_count += 1
	if v ~= false then
		busted_fail(`expected false, got {busted_show(v)}`)
	end
end

function assert.is_falsy(v)
	busted_count += 1
	if v then
		busted_fail(`expected a falsy value, got {busted_show(v)}`)
	end
end

function assert.is_nil(v)
	busted_count += 1
	if v ~= nil then
		busted_fail(`expected nil, got {busted_show(v)}`)
	end
end

function assert.not_table(v)
	busted_count += 1
	if type(v) == "table" then
		busted_fail("expected something other than a table")
	end
end

function assert.error(f, ...)
	busted_count += 1
	if pcall(f, ...) then
		busted_fail("expected an error")
	end
end

function assert.not_error(f, ...)
	busted_count += 1
	local ok, err = pcall(f, ...)
	if not ok then
		busted_fail(`expected no error, got {busted_show(err)}`)
	end
end

local function busted_run(block, path, outer_befores, outer_afters)
	local befores = table.clone(outer_befores)
	for _, f in block.befores do
		table.insert(befores, f)
	end
	local afters = table.clone(outer_afters)
	for i = #block.afters, 1, -1 do
		table.insert(afters, 1, block.afters[i])
	end
	for _, test in block.tests do
		if test.describe then
			local inner = { befores = {}, afters = {}, tests = {} }
			local outer = busted_current
			busted_current = inner
			test.body()
			busted_current = outer
			busted_run(inner, `{path}{test.describe} / `, befores, afters)
		else
			busted_count = 0
			busted_finally = {}
			local ok, err = pcall(function()
				for _, f in befores do
					f()
				end
				test.body()
			end)
			for _, f in afters do
				pcall(f)
			end
			for _, f in busted_finally do
				pcall(f)
			end
			if ok then
				print("ok", `{path}{test.name}`, busted_count)
			else
				print("FAIL", `{path}{test.name}`, tostring(err))
			end
		end
	end
end

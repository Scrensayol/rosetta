-- calls Luau turns into FASTCALL or folds when the arguments are constants: math, bit32, string
-- and buffer, with arguments at their edges, and the same builtins through locals and upvalues
print(math.floor(-2.5), math.ceil(-2.5), math.abs(-0), math.max(1, 5, 3), math.min(-1, -5), math.sign(-3))
print(math.floor(2 ^ 31 + 0.5), math.fmod(-7, 3), math.fmod(7, -3), math.clamp(15, 0, 10), math.round(-2.5))
print(bit32.band(0xff, -1), bit32.bor(1, 2, 4), bit32.bxor(5, 3), bit32.bnot(0), bit32.lshift(1, 31))
print(bit32.rshift(-1, 28), bit32.arshift(-16, 2), bit32.extract(0xabcd, 4, 8), bit32.replace(0, 7, 4, 3))
print(bit32.countlz(1), bit32.countrz(8), bit32.lrotate(1, 33), bit32.btest(6, 1), bit32.byteswap(0x01020304))
print(string.byte("ABC", 1, -1), string.char(72, 105), ("x"):rep(3, "-"), ("Hello"):sub(-3), ("Hi"):len())
print(("%5.2f|%-4d|%x|%s"):format(3.14159, 7, 255, "s"), ("a,b,,c"):split(",")[3] == "", #("a,b,,c"):split(","))
print(("hello world"):gsub("o", "0"), ("abc"):gsub("%w", "%0%0"), ("x=1 y=2"):gsub("(%w)=(%w)", "%2=%1"))
print(("key=val"):match("(%w+)=(%w+)"), ("  trim  "):match("^%s*(.-)%s*$"), ("a1b2"):find("%d"))
print(tonumber("0x10"), tonumber("10", 2), tonumber(" 5 "), tonumber("5e2"), tonumber("abc"), tonumber("z", 36))
print(type(nil), typeof(1), select("#", 1, nil, nil), select(-1, "a", "b"), rawlen({ 1, 2, 3 }), rawlen("abcd"))

local buf = buffer.create(16)
buffer.writeu8(buf, 0, 255)
buffer.writei16(buf, 1, -2)
buffer.writef32(buf, 4, 1.5)
buffer.writestring(buf, 8, "rosetta")
print(buffer.readu8(buf, 0), buffer.readi8(buf, 0), buffer.readi16(buf, 1), buffer.readu16(buf, 1))
print(buffer.readf32(buf, 4), buffer.readstring(buf, 8, 7), buffer.len(buf), buffer.tostring(buf):byte(9))

-- the same builtins through locals and upvalues, which compile differently
local floor, band, sub = math.floor, bit32.band, string.sub
local function viaUpvalues(v)
	return floor(v), band(v, 3), sub("abcdef", v, v + 1)
end
print(viaUpvalues(2), viaUpvalues(5))

-- a global that shadows a builtin library stops it being treated as one
local saved = math
local function sneaky()
	local math = { floor = function()
		return "shadowed"
	end }
	return math.floor(1.5)
end
print(sneaky(), saved.floor(1.5))

local t = { 3, 1, 2 }
table.sort(t, function(x, y)
	return x > y
end)
table.insert(t, 1, 0)
print(table.concat(t, ","), table.remove(t), table.find(t, 3), #table.create(3, "x"), table.unpack({ 1, 2, 3 }, 2))

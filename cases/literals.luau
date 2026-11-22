-- literals that have to come back exactly: escapes, control bytes, long brackets that contain
-- their own closers, numbers at the edges of what %.17g and hex can say, and -0
print("tab\there", "nl\\n", "quote\"s", 'single\'s', "bell\a", "nul[\0]", "del\127", "hi\255\128")
print(#"\0\0\0", ("\0x"):byte(1, -1))
print("\u{48}\u{E9}\u{1F600}", #"\u{1F600}")
print("\x41\x42", "\65\066\0671", "line1\
line2")
print([[long ]] .. [==[with ]] and ]=] inside]==], [[
leading newline dropped]])
print(`interp {1 + 1} \{not} {"{}"}`)

local nums = {
	0, -0, 1 / -math.huge, 0.1, 0.1 + 0.2, 1e-310, 5e-324, 1.7976931348623157e308,
	2 ^ 53, 2 ^ 53 + 1, 9007199254740993, 0x7fffffff, 0xffffffffffff, 0x10, 1e15, 1e16,
	123456789012345678, 3.14159265358979, 1 / 3, -1 / 3, 100, 1e2, 0.5e1,
}
for i, n in ipairs(nums) do
	print(i, n, string.format("%.17g", n), 1 / n == math.huge, 1 / n == -math.huge)
end

-- -0 survives arithmetic only if the constant does
local z = -0
print(1 / z, 1 / (z * 1), 1 / (0 * -1), math.max(-0, 0), tostring(-0.0))

-- strings that look like other things
local keys = { "end", "nil", "1", "a b", "", "\n", "x.y", "__index" }
local t = {}
for i, k in ipairs(keys) do
	t[k] = i
end
for _, k in ipairs(keys) do
	print(string.format("%q", k), t[k])
end
print(t["end"], t["nil"], t["1"], t[1])

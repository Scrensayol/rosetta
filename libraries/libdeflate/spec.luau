-- LibDeflate 1.0.2-release (SafeteeWoW/LibDeflate afc3b78): DEFLATE, zlib and preset-dictionary
-- compression in 3,600 lines of pure Lua, with bit operations done in arithmetic and big
-- functions full of locals and upvalues. Its upstream tests need io and luaunit, so this spec
-- is Rosetta's own: every compressor setting round-trips a fixed set of inputs, and the
-- compressed bytes are checksummed, so output that decompresses but differs still shows
local LibDeflate = require("libdeflate")

-- the inputs: fixed strings, plus bytes from a linear congruential generator, so the same
-- inputs come out on every run without math.random
local function noise(seed, length, alphabet)
	local parts = table.create(length)
	local x = seed
	for i = 1, length do
		x = (x * 1103515245 + 12345) % 2147483648
		parts[i] = string.char(x // 65536 % alphabet)
	end
	return table.concat(parts)
end

local text = string.rep("the quick brown fox jumps over the lazy dog. ", 40)
local inputs = {
	{ "empty", "" },
	{ "one byte", "a" },
	{ "every byte", (function()
		local t = {}
		for i = 0, 255 do
			t[#t + 1] = string.char(i)
		end
		return table.concat(t)
	end)() },
	{ "repeated text", text },
	{ "one byte repeated", string.rep("z", 3000) },
	{ "noise", noise(1, 1500, 256) },
	{ "noise over four bytes", noise(2, 2500, 4) },
	{ "text with noise", text:sub(1, 900) .. noise(3, 300, 256) .. text:sub(200, 1100) },
}

describe("Adler32", function()
	it("matches known checksums", function()
		assert.equal(LibDeflate:Adler32(""), 1)
		assert.equal(LibDeflate:Adler32("Wikipedia"), 300286872)
		assert.equal(LibDeflate:Adler32("abc"), 38600999)
	end)
	for _, input in inputs do
		it(input[1], function()
			print("adler32", input[1], LibDeflate:Adler32(input[2]))
		end)
	end
end)

local function check(name, compress, decompress, input)
	local compressed = compress(input)
	print(name, #compressed, LibDeflate:Adler32(compressed))
	local back, left = decompress(compressed)
	assert.equal(back, input)
	assert.equal(left, 0)
end

describe("Deflate", function()
	for level = 0, 9 do
		describe(`level {level}`, function()
			for _, input in inputs do
				it(input[1], function()
					check(`deflate {level} {input[1]}`, function(s)
						return LibDeflate:CompressDeflate(s, { level = level })
					end, function(s)
						return LibDeflate:DecompressDeflate(s)
					end, input[2])
				end)
			end
		end)
	end
	for _, strategy in { "fixed", "huffman_only", "dynamic" } do
		describe(strategy, function()
			for _, input in inputs do
				it(input[1], function()
					check(`deflate {strategy} {input[1]}`, function(s)
						return LibDeflate:CompressDeflate(s, { level = 5, strategy = strategy })
					end, function(s)
						return LibDeflate:DecompressDeflate(s)
					end, input[2])
				end)
			end
		end)
	end
end)

describe("Zlib", function()
	for _, level in { 1, 6, 9 } do
		describe(`level {level}`, function()
			for _, input in inputs do
				it(input[1], function()
					check(`zlib {level} {input[1]}`, function(s)
						return LibDeflate:CompressZlib(s, { level = level })
					end, function(s)
						return LibDeflate:DecompressZlib(s)
					end, input[2])
				end)
			end
		end)
	end
end)

describe("Dictionary", function()
	local source = text:sub(1, 300) .. noise(4, 200, 256)
	local dictionary = LibDeflate:CreateDictionary(source, #source, LibDeflate:Adler32(source))
	it("refuses a wrong checksum", function()
		assert.error(function()
			LibDeflate:CreateDictionary(source, #source, 12345)
		end)
	end)
	for _, input in inputs do
		it(input[1], function()
			check(`deflate dict {input[1]}`, function(s)
				return LibDeflate:CompressDeflateWithDict(s, dictionary, { level = 7 })
			end, function(s)
				return LibDeflate:DecompressDeflateWithDict(s, dictionary)
			end, input[2])
			check(`zlib dict {input[1]}`, function(s)
				return LibDeflate:CompressZlibWithDict(s, dictionary, { level = 7 })
			end, function(s)
				return LibDeflate:DecompressZlibWithDict(s, dictionary)
			end, input[2])
		end)
	end
end)

describe("Corrupt input", function()
	local compressed = LibDeflate:CompressDeflate(text, { level = 6 })
	local zlib = LibDeflate:CompressZlib(text, { level = 6 })
	it("truncated deflate", function()
		for _, cut in { 1, 5, 20, #compressed // 2, #compressed - 1 } do
			print("truncated", cut, LibDeflate:DecompressDeflate(compressed:sub(1, cut)))
		end
	end)
	it("bad zlib checksum", function()
		local broken = zlib:sub(1, -2) .. string.char((zlib:byte(-1) + 1) % 256)
		assert.is_nil(LibDeflate:DecompressZlib(broken))
	end)
	it("bad zlib header", function()
		assert.is_nil(LibDeflate:DecompressZlib("\120\157" .. zlib:sub(3)))
	end)
	it("bad arguments", function()
		assert.error(function()
			LibDeflate:CompressDeflate(1)
		end)
		assert.error(function()
			LibDeflate:CompressDeflate("x", { level = 10 })
		end)
		assert.error(function()
			LibDeflate:CompressDeflate("x", { strategy = "nope" })
		end)
		assert.error(function()
			LibDeflate:DecompressDeflate({})
		end)
	end)
end)

describe("Encodings", function()
	for _, input in inputs do
		it(input[1], function()
			local printable = LibDeflate:EncodeForPrint(input[2])
			print("print", input[1], #printable, LibDeflate:Adler32(printable))
			assert.equal(LibDeflate:DecodeForPrint(printable), input[2])
			local addon = LibDeflate:EncodeForWoWAddonChannel(input[2])
			assert.is_nil(addon:find("\0", 1, true))
			assert.equal(LibDeflate:DecodeForWoWAddonChannel(addon), input[2])
			local chat = LibDeflate:EncodeForWoWChatChannel(input[2])
			print("chat", input[1], #chat, LibDeflate:Adler32(chat))
			assert.equal(LibDeflate:DecodeForWoWChatChannel(chat), input[2])
		end)
	end
	it("a custom codec", function()
		local codec = LibDeflate:CreateCodec("\0\1", "\2", "\3")
		for _, input in inputs do
			local encoded = codec:Encode(input[2])
			assert.is_nil(encoded:find("[%z\1]"))
			assert.equal(codec:Decode(encoded), input[2])
		end
		assert.is_nil(LibDeflate:CreateCodec("\0", "\0", ""))
	end)
end)

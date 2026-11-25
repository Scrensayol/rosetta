-- a function at Luau's limits: 190 locals (the cap is 200), a call with 60 arguments on top of
-- them (registers stop at 255), a 100-entry constructor and closures over the first and last
-- local. The compiler fits it; a decompiler that adds temporaries or declares more locals than
-- the original had writes something that doesn't recompile
local source = {}
for i = 1, 200 do
	source[i] = i * 7 % 101
end
local function values(from)
	return table.unpack(source, from, from + 18)
end
local function pick(...)
	local sum = 0
	for i = 1, select("#", ...) do
		sum += select(i, ...) * i
	end
	return select("#", ...), sum
end

local function wide()
	local a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16, a17, a18, a19 = values(1)
	local a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, a31, a32, a33, a34, a35, a36, a37, a38 = values(20)
	local a39, a40, a41, a42, a43, a44, a45, a46, a47, a48, a49, a50, a51, a52, a53, a54, a55, a56, a57 = values(39)
	local a58, a59, a60, a61, a62, a63, a64, a65, a66, a67, a68, a69, a70, a71, a72, a73, a74, a75, a76 = values(58)
	local a77, a78, a79, a80, a81, a82, a83, a84, a85, a86, a87, a88, a89, a90, a91, a92, a93, a94, a95 = values(77)
	local a96, a97, a98, a99, a100, a101, a102, a103, a104, a105, a106, a107, a108, a109, a110, a111, a112, a113, a114 = values(96)
	local a115, a116, a117, a118, a119, a120, a121, a122, a123, a124, a125, a126, a127, a128, a129, a130, a131, a132, a133 = values(115)
	local a134, a135, a136, a137, a138, a139, a140, a141, a142, a143, a144, a145, a146, a147, a148, a149, a150, a151, a152 = values(134)
	local a153, a154, a155, a156, a157, a158, a159, a160, a161, a162, a163, a164, a165, a166, a167, a168, a169, a170, a171 = values(153)
	local a172, a173, a174, a175, a176, a177, a178, a179, a180, a181, a182, a183, a184, a185, a186, a187, a188, a189, a190 = values(172)
	local function corners()
		a1 += 1
		return a1 + a190, a95
	end
	print(corners())
	print(a1)
	-- one sum per declaration line: a 190-term chain would hold a temporary at every level
	print(
		a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + a13 + a14 + a15 + a16 + a17 + a18 + a19,
		a20 + a21 + a22 + a23 + a24 + a25 + a26 + a27 + a28 + a29 + a30 + a31 + a32 + a33 + a34 + a35 + a36 + a37 + a38,
		a39 + a40 + a41 + a42 + a43 + a44 + a45 + a46 + a47 + a48 + a49 + a50 + a51 + a52 + a53 + a54 + a55 + a56 + a57,
		a58 + a59 + a60 + a61 + a62 + a63 + a64 + a65 + a66 + a67 + a68 + a69 + a70 + a71 + a72 + a73 + a74 + a75 + a76,
		a77 + a78 + a79 + a80 + a81 + a82 + a83 + a84 + a85 + a86 + a87 + a88 + a89 + a90 + a91 + a92 + a93 + a94 + a95,
		a96 + a97 + a98 + a99 + a100 + a101 + a102 + a103 + a104 + a105 + a106 + a107 + a108 + a109 + a110 + a111 + a112 + a113 + a114,
		a115 + a116 + a117 + a118 + a119 + a120 + a121 + a122 + a123 + a124 + a125 + a126 + a127 + a128 + a129 + a130 + a131 + a132 + a133,
		a134 + a135 + a136 + a137 + a138 + a139 + a140 + a141 + a142 + a143 + a144 + a145 + a146 + a147 + a148 + a149 + a150 + a151 + a152,
		a153 + a154 + a155 + a156 + a157 + a158 + a159 + a160 + a161 + a162 + a163 + a164 + a165 + a166 + a167 + a168 + a169 + a170 + a171,
		a172 + a173 + a174 + a175 + a176 + a177 + a178 + a179 + a180 + a181 + a182 + a183 + a184 + a185 + a186 + a187 + a188 + a189 + a190
	)
	print(pick(a1, a4, a7, a10, a13, a16, a19, a22, a25, a28, a31, a34, a37, a40, a43,
		a46, a49, a52, a55, a58, a61, a64, a67, a70, a73, a76, a79, a82, a85, a88,
		a91, a94, a97, a100, a103, a106, a109, a112, a115, a118, a121, a124, a127, a130, a133,
		a136, a139, a142, a145, a148, a151, a154, a157, a160, a163, a166, a169, a172, a175, a178))
	local t = {
		a91, a92, a93, a94, a95, a96, a97, a98, a99, a100, a101, a102, a103, a104, a105, a106, a107, a108, a109, a110,
		a111, a112, a113, a114, a115, a116, a117, a118, a119, a120, a121, a122, a123, a124, a125, a126, a127, a128, a129, a130,
		a131, a132, a133, a134, a135, a136, a137, a138, a139, a140, a141, a142, a143, a144, a145, a146, a147, a148, a149, a150,
		a151, a152, a153, a154, a155, a156, a157, a158, a159, a160, a161, a162, a163, a164, a165, a166, a167, a168, a169, a170,
		a171, a172, a173, a174, a175, a176, a177, a178, a179, a180, a181, a182, a183, a184, a185, a186, a187, a188, a189, a190,
	}
	print(#t, t[1], t[100])
	return a190, a189
end
print(wide())

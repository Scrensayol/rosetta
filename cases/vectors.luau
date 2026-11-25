-- vectors: vector.create with constant arguments folds into a vector constant, which has to come
-- back as something vanilla Luau runs (Roblox spells it Vector3.new, plain Luau doesn't have
-- that), plus vector arithmetic, components and the vector library, which compile to fast calls
local function show(v)
	return string.format("%g,%g,%g", v.x, v.y, v.z)
end

local up = vector.create(0, 1, 0)
local unit = vector.create(1, 1, 1)
print(show(up), show(unit), typeof(up), up == vector.create(0, 1, 0))
print(show(vector.create(-0, 0.5, 1e300)), show(vector.create(1 / 3, 2 ^ 20, -7)))

-- arithmetic with vectors and numbers, and unary minus
local a, b = vector.create(1, 2, 3), vector.create(4, 5, 6)
print(show(a + b), show(a - b), show(a * b), show(b / a), show(a * 2), show(3 / b), show(-a), show(b // a))

-- components, and arguments known only at run time
local n = #"abcd"
local v = vector.create(n, n * 2, n // 3)
print(v.x, v.y, v.z, v.X, v.Y, v.Z)

-- the vector library
print(vector.magnitude(vector.create(3, 4, 0)), show(vector.normalize(vector.create(0, 0, 9))))
print(vector.dot(a, b), show(vector.cross(a, b)), show(vector.floor(vector.create(1.5, -1.5, 2))))
print(show(vector.ceil(vector.create(1.2, -1.2, 0))), show(vector.abs(vector.create(-1, 2, -3))))
print(show(vector.sign(vector.create(-5, 0, 5))), show(vector.clamp(vector.create(5, -5, 0), vector.zero, vector.one)))
print(show(vector.max(a, b, vector.create(9, 0, 0))), show(vector.min(a, b)), vector.angle(vector.create(1, 0, 0), up))
print(show(vector.zero), show(vector.one))

-- vectors as table keys and in comparisons
local seen = {}
seen[vector.create(1, 2, 3)] = "found"
print(seen[a], a == vector.create(1, 2, 3), a ~= b)

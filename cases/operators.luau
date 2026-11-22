-- precedence and arithmetic corners a printer has to parenthesize or keep apart: unary minus
-- against ^, not against ==, right-associative .. and ^, and Luau's floor division and modulo with
-- negative and fractional operands
local values = { 7, -7, 2.5, -2.5, 0.5, 3 }
for _, a in ipairs(values) do
	for _, b in ipairs({ 2, -2, 0.5, 3 }) do
		print(a, b, a // b, a % b, a / b, -a ^ b == -(a ^ b), (-a) ^ 2, a - -b)
	end
end

local x, y = 2, 3
print(-x ^ 2, (-x) ^ 2, 2 ^ -1, 2 ^ 3 ^ 2, (2 ^ 3) ^ 2)
print(not x == y, not (x == y), not not x, - -x, -(-x))
print(1 .. 2, 1 .. 2 .. 3, "a" .. 1 + 2, ("a" .. 1) .. 2)
print(x < y == true, (x < y) == true, x + y * 2, (x + y) * 2, x * y % 4, x * (y % 4))
print(#"abc" + 1, #{ 1, 2 } * 2, -#"ab", #("a" .. "bc"))
print(7 // 0, -7 // 0, 0 / 0 ~= 0 / 0, -7 % math.huge, 7 % -math.huge)
print(2 ^ 53 + 1, 2 ^ 63, -2 ^ 63, 1e308 * 10, -1e308 * 10)

-- the same expressions on values the compiler can't fold
local function id(v)
	return v
end
local p, q = id(-7), id(2)
print(p // q, p % q, -p ^ q, (p .. q), p < q and "lt" or "ge", not (p < q))

-- comparisons the printer might swap or merge
local function order(a, b)
	return a < b, a > b, b < a, a <= b, not (a > b), a == b, a ~= b
end
print(order(1, 2))
print(order("a", "b"))
print(order(2, 2))

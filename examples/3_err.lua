-- comentários são ignorados

local a, b, c

a, b = 1, 2
b, c = a + b, a - b

function foo(a)
  return a + 1
end

-- chamadas de função são statements
foo(a)

-- mas também expressões
a = foo(b)

do
  local x, y
  c = a * x ^ b + y
end

-- escopo já terminou
x = c

if c then
  local w, z
  w = z + c
elseif a then
  foo(c)
else
  b = "foobar"
end

-- outros erros de escopo também são detectado
a = w

while c ~= 0 do
  c = c / 2
end

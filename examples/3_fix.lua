-- comentários são ignorados

local a, b, c

-- variáveis extras do lado esquerdo são válidas, recebem nil
a, b, c = true, false

-- expressões extras do lado direito são ignoradas
a, b = 10.0, -.2, 3

-- chamadas de função são statements
c(a)

-- mas também expressões
a = c(b)

do
  local x, y
  c = a * x ^ b + y
end

-- escopo já terminou
--x = c

if c then
  local w, x
  w = x + c
end

-- outros erros de escopo também são detectado
--a = w

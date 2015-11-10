local a, i = 1, 0
while i < 10 do
  local j = a * (i + 1)
  a = a * j
  i = i + 1
end

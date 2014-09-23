-- script.lua
-- Receives a table, returns the sum of its components.
print("The table the script received has:")
x = 0
for i = 1, #foo do
  print(i, foo[i])
  x = x + foo[i]
end
print("Returning data back to Rust")
return x

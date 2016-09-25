def take list
  p "#{list[0]}"
end

list = [10, 5, 4]
listb = list
listc = listb

# list.push(42)

p "#{list}"
p "#{listb}"
p "#{listc}"

take list
take list

# Running
# cd examples/ruby/
# ruby ownership.rb

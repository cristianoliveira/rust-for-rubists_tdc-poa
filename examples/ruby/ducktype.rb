class Foo
  def hi
    "foo"
  end
end

class Bar
  def hi
    "bar"
  end
end

def hello(f)
  puts f.hi
end

foo = Foo.new
hello(foo)

bar = Bar.new
hello(bar)

# Running
# cd examples/ruby/
# ruby dicktype.rb

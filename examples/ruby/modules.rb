module Greeting
  def hi
    "bar"
  end
end

class Foo
  include Greeting
end

f = Foo.new
f.hi

# Running
# cd examples/ruby/
# ruby modules.rb

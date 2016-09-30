class A
  def foo; end
end

class B < A
  def bar; end
end

class C < B
  def zas; end
end

p "class C parent lookup: #{C.ancestors}"

code = "2*7+3"
puts RubyVM::InstructionSequence.compile(code).disassemble

# Running
# cd examples/ruby/
# ruby ruby_calc.rb

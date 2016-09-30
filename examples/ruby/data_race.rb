words = ["Hello", "world", "From", "Ruby"]

words.map do |w|
  Thread.new do
    puts "#{w}"
  end.join()
end

# words.each_with_index do |w, i|
#   Thread.new do
#     # This is weird in Ruby but it's closer to the println! macro
#     # usage in the Rust example.
#     puts "#{words[i]}"
#   end.join()
# end
#

# Running
# cd example/ruby/
# ruby race_condition.rb

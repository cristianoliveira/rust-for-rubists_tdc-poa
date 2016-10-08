words = ["Hello", "world", "From", "Ruby"]

100.times.map do
  words.each_with_index.map do |w, i|
    puts "#{i} #{words[i]}"
  end
end

# 100.times.map do
#   Thread.new do
#     words.each_with_index.map do |w, i|
#       puts "#{i} #{words[i]}"
#     end
#   end
# end.each(&:join)

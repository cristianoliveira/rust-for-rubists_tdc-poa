words = ["Hello", "world", "From", "Ruby"]

10.times.map do
  Thread.new do
    words.each_with_index.map do |w, i|
      puts "#{i} #{words[i]}"
    end
  end
end.each(&:join)

# words.each_with_index.map do |w, i|
#   Thread.new do
#     words.push(" lol ")
#     puts "#{i} #{words[i]}"
#   end
# end.each(&:join)


# Running
# cd example/ruby/
# ruby race_condition.rb

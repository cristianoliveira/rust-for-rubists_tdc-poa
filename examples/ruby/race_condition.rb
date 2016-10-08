words = ["Hello", "world", "From", "Ruby"]

words.each_with_index.map do |w, i|
  Thread.new do
    words.push(" lol ")
    puts "#{i} #{words[i]}"
  end
end.each(&:join)


# Running
# cd example/ruby/
# ruby race_condition.rb

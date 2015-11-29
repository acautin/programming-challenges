STDOUT.sync = true # DO NOT REMOVE
# Save humans, destroy zombies!


# game loop
loop do
    self_x, self_y = gets.split(" ").collect {|x| x.to_i}
    human_count = gets.to_i
    i = 0;
    human_x = []
    human_y = []
    human_count.times do
        human_id, human_x[i], human_y[i] = gets.split(" ").collect {|x| x.to_i}
        i = i + 1
    end
    zombie_count = gets.to_i
    target_x = self_x
    target_y = self_y
    min_dist = 20000
    zombie_count.times do
        zombie_id, zombie_x, zombie_y, zombie_xnext, zombie_ynext = gets.split(" ").collect {|x| x.to_i}
        i = 0
        human_count.times do
            x = human_x[i] - self_x
            y = human_y[i] - self_y
            self_dist = Math.sqrt(x*x + y*y)
            turns = (self_dist/1000).ceil
            x = human_x[i] - zombie_xnext
            y = human_y[i] - zombie_ynext
            human_dist = Math.sqrt(x*x + y*y)
            zombie_turns = (human_dist/400).ceil
            can_save = turns <= zombie_turns
            if can_save and human_dist < min_dist
                min_dist = human_dist
                target_x = zombie_xnext
                target_y = zombie_ynext
            end
            i = i + 1
        end
    end
    
    # Write an action using puts
    # To debug: STDERR.puts "Debug messages..."
    
    puts "#{target_x} #{target_y}" # Your destination coordinates
end

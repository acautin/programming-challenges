STDOUT.sync = true # DO NOT REMOVE
# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.

class Point
  attr_accessor :x, :y
  def initialize(x, y)
    @x = x
    @y = y
  end
end

target1 = Point.new(0, 0)
target2 = Point.new(0, 0)
surface_n = gets.to_i # the number of points used to draw the surface of Mars.
land_x, land_y = gets.split(" ").collect{|x| x.to_i}
(surface_n - 1).times do
  # land_x: X coordinate of a surface point. (0 to 6999)
  # land_y: Y coordinate of a surface point. By linking all the points together in a sequential fashion, you form the surface of Mars.
  prev_x, prev_y = land_x, land_y
  land_x, land_y = gets.split(" ").collect{|x| x.to_i}
  if land_y == prev_y
    target1.x = prev_x
    target1.y = land_y
    target2.x = land_x
    target2.y = land_y
  end
end

#puts "the target from #{target1.x} #{target1.y} -> #{target2.x} #{target2.y}"

# game loop
loop do
# h_speed: the horizontal speed (in m/s), can be negative.
# v_speed: the vertical speed (in m/s), can be negative.
# fuel: the quantity of remaining fuel in liters.
# rotate: the rotation angle in degrees (-90 to 90).
# power: the thrust power (0 to 4).
  x, y, h_speed, v_speed, fuel, rotate, power = gets.split(" ").collect {|x| x.to_i}
  

  # Write an action using puts
  # To debug: STDERR.puts "Debug messages..."
  add = 0
  if (v_speed < -30 && (y - target1.y) < 1000) || ((y - target1.y) < 500 && (x < target1.x || x > target2.x))
    add = 1
  end
  if x < target1.x
    if h_speed > 50
      puts "30 4"
    elsif h_speed > 30
      puts "0 #{3+add}"
    else
      puts "-30 4"
    end
  elsif x < target2.x
    if h_speed > -2 && h_speed < 2
      puts "0 #{3+add}"
    elsif h_speed > 0
      puts "30 4"
    else
      puts "-30 4"
    end
  else
    if h_speed < -50
      puts "-30 4"
    elsif h_speed < -30
      puts "0 #{3+add}"
    else
      puts "30 4"
    end
  end
end

//extern crate rand;

use std::io;
//use rand::Rng;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    //let width = parse_input!(inputs[0], i32);
    let height = parse_input!(inputs[1], i32);
    let my_id = parse_input!(inputs[2], i32);

    let mut pos_x = 0;
    let mut pos_y = 0;

    if my_id == 1 {
        pos_x = 12;
        pos_y = 10;
    }
    let mut range = 3;

    // game loop
    loop {
        // TODO: Learn how to create [vec![]; 10]
        let mut danger = [[0; 11]; 13];
        let mut free = [['.'; 11]; 13];
        let mut boxes_left = false;
        for i in 0..height as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim_right().to_string();
            for (j, c) in row.chars().enumerate() {
                if is_box(c) {
                    boxes_left = true;
                }
                free[j][i] = c;
            }
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entities = parse_input!(input_line, i32);
        let mut bombs = vec![];
        //TODO: Bomb if will trap an enemy.
        let mut enemies = vec![];        
        for _ in 0..entities as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_type = parse_input!(inputs[0], i32);
            let owner = parse_input!(inputs[1], i32);
            let x = parse_input!(inputs[2], usize);
            let y = parse_input!(inputs[3], usize);
            let param_1 = parse_input!(inputs[4], i32);
            let param_2 = parse_input!(inputs[5], i32);

            if entity_type == 0 {
                if owner == my_id { 
                    pos_x = x;
                    pos_y = y;
                    range = param_2;
                } else {
                    enemies.push((x, y, range));
                }
            } else if entity_type == 1 {
                // print_err!("Bomb {} {} timer {}", x, y, param_1);
                free[x][y] = 'B';
                //TODO: Use insertion sort as seems to be sorted now by timer, but we shouldn't count on that ...
                bombs.push((x, y, param_1, param_2));
            } else if entity_type == 2 {
                // print_err!("Power up on {} {} type {}", x, y, param_1);
                free[x][y] = 'P';
            }

            /*
            if entity_type == 2 {
                if dst(x, pos_x, y, pos_y) < dist {
                    dist = dst(x, pos_x, y, pos_y);
                    target_x = x;
                    target_y = y;
                }
            }
            if entity_type == 2 && target_x == x && target_y == y {
                found = true;
            }
            */
        }
        for (x, y, timer, range) in bombs {
            add_bomb(x, y, &mut danger, free, timer, range);            
        }
        // TODO: It is important to pick up type 2 bombs when we can carry < 3
        /*
        if target_x < 0 {
            target_x = rand::thread_rng().gen_range(0, 13);
            target_y = rand::thread_rng().gen_range(0, 11);
            let (safe_x, safe_y) = safe(target_x, target_y, danger, free);
            target_x = safe_x;
            target_y = safe_y;
        }
        */

        //Check if I am in danger.
        if danger[pos_x][pos_y] != 0 {
            // Find the closest safe place.
            let mut safe_cell = safe_target(pos_x, pos_y, danger, free, boxes_left);
            if boxes_left && safe_cell == (pos_x, pos_y) {
                safe_cell = safe_target(pos_x, pos_y, danger, free, false);
            }
            if is_box_in_range(pos_x, pos_y, danger, free, range) {
                add_bomb(pos_x, pos_y, &mut danger, free, 8, range);
                let bombed_safe_cell = safe_target(pos_x, pos_y, danger, free, false);
                if bombed_safe_cell != (pos_x, pos_y) {
                    println!("BOMB {} {}", bombed_safe_cell.0, bombed_safe_cell.1);
                } else {
                    println!("MOVE {} {}", safe_cell.0, safe_cell.1);
                }
            } else {
                println!("MOVE {} {}", safe_cell.0, safe_cell.1);
            } 
        } else {
            if boxes_left {
                let mut safe_cell = safe_target(pos_x, pos_y, danger, free, true);
                if safe_cell == (pos_x, pos_y) {
                    safe_cell = safe_target(pos_x, pos_y, danger, free, false);               
                }
                
                if is_box_in_range(pos_x, pos_y, danger, free, range) {
                    add_bomb(pos_x, pos_y, &mut danger, free, 8, range);
                    for (enemy_x, enemy_y, enemy_range) in enemies {
                        add_bomb(enemy_x, enemy_y, &mut danger, free, 8, enemy_range)
                    }
                    let mut bombed_safe_cell = safe_target(pos_x, pos_y, danger, free, true);
                    if bombed_safe_cell == (pos_x, pos_y) {
                        bombed_safe_cell = safe_target(pos_x, pos_y, danger, free, false);
                    }
                    if bombed_safe_cell != (pos_x, pos_y) {
                        println!("BOMB {} {}", bombed_safe_cell.0, bombed_safe_cell.1);   
                    } else {
                        println!("MOVE {} {}", safe_cell.0, safe_cell.1);
                    }
                } else {
                    println!("MOVE {} {}", safe_cell.0, safe_cell.1);
                }
            } else {
                println!("MOVE {} {} MOJITO TIME", pos_x, pos_y);
            }
        }
    }
}

// Manhattan distance as there is no diagonal moves.
/*
fn dst(x1:i32, y1:i32, x2:i32, y2:i32) -> i32 {
    return (x2-x1).abs() + (y2-y1).abs();
}
*/

// TODO: return next box without bomb.
fn safe_target(x: usize, y: usize, danger:[[i32; 11]; 13], free:[[char; 11]; 13], find_box: bool) -> (usize, usize) {
    let c0 = (x, y, 2, (x, y));
    let mut q = vec![c0];
    let mut visited = [[false; 11]; 13];
    while let Some(cell) = q.pop() {
        if visited[cell.0][cell.1] {
            continue;
        }
        if danger[cell.0][cell.1] == 0 && is_walkable(free[cell.0][cell.1]) && cell != c0 {
            // print_err!("No danger found on {} {} steps {}", cell.0, cell.1, cell.2 - 2);
            if !find_box || is_adj_box(cell.0, cell.1, danger, free) || free[cell.0][cell.1] == 'P' || cell.2 > 20 {
                // print_err!("Returning...");
                return cell.3;
            }
            // print_err!("But we are looking for something to blow up...");
        }
        let mut first_step = cell.3;
        // LEFT
        if cell.0 != 0 && is_step_safe(cell.0 - 1, cell.1, danger, free, cell.2) {
            if cell.2 == 2 {
                first_step = (cell.0 - 1, cell.1);
            }
            q.insert(0, (cell.0 - 1, cell.1, cell.2 + 1, first_step));
        }
        // DOWN
        if cell.1 != 10 && is_step_safe(cell.0, cell.1 + 1, danger, free, cell.2) {
            if cell.2 == 2 {
                first_step = (cell.0, cell.1 + 1);
            }
            q.insert(0, (cell.0, cell.1 + 1, cell.2 + 1, first_step));
        }
        // RIGHT
        if cell.0 != 12 && is_step_safe(cell.0 + 1, cell.1, danger, free, cell.2) {
            if cell.2 == 2 {
                first_step = (cell.0 + 1, cell.1);
            }
            q.insert(0, (cell.0 + 1, cell.1, cell.2 + 1, first_step));
        }
        // UP
        if cell.1 != 0 && is_step_safe(cell.0, cell.1 - 1, danger, free, cell.2) {
            if cell.2 == 2 {
                first_step = (cell.0, cell.1 - 1);
            }
            q.insert(0, (cell.0, cell.1 - 1, cell.2 + 1, first_step));
        }
        visited[cell.0][cell.1] = true;
    }
    // print_err!("No safe cell found");
    return (x, y);
}

// TODO: Maybe we should take into account to stop at power ups...
fn add_bomb(x: usize, y: usize, danger: &mut [[i32; 11]; 13], free: [[char; 11]; 13], mut timer: i32, range: i32) {
    if danger[x][y] != 0 {
        timer = danger[x][y];
    }
    let mut min32 = x as i32 - (range - 1);
    let mut min = 0;
    if min32 > 0 {
        min = min32 as usize;
    }
    let mut max = x + range as usize;
    if max > 13 {
        max = 13;
    }
    for j in (min..x).rev() {
        danger[j][y] = set_timer(j, y, danger, timer);
        if !is_walkable(free[j][y]) {
            break;
        }
    }
    for j in x..max {
        danger[j][y] = set_timer(j, y, danger, timer);
        if j != x && !is_walkable(free[j][y]) {
            break;
        }
    }
    min32 = y as i32 - (range - 1);
    min = 0;
    if min32 > 0 {
        min = min32 as usize;
    }
    max = y + range as usize;
    if max > 11 {
        max = 11;
    }
    for j in (min..y).rev() {
        danger[x][j] = set_timer(x, j, danger, timer);
        if !is_walkable(free[x][j]) {
            break;
        }
    }
    for j in y..max {
        danger[x][j] = set_timer(x, j, danger, timer);
        if j != y && !is_walkable(free[x][j]) {
            break;
        }
    }
}

fn set_timer(x: usize, y: usize, danger: &mut [[i32; 11]; 13], timer: i32) -> i32 {
    if danger[x][y] == 0 {
        return timer;
    }
    return danger[x][y] * 10 + timer;
}

fn is_adj_box(x: usize, y: usize, danger: [[i32; 11]; 13], free: [[char; 11]; 13]) -> bool {
    if x != 0 && danger[x-1][y] == 0 && is_box(free[x-1][y]) {
        return true;
    } else if x != 12 && danger[x+1][y] == 0 && is_box(free[x+1][y]) {
        return true;
    } else if y != 0 && danger[x][y-1] == 0 && is_box(free[x][y-1]) {
        return true;
    } else if y != 10 && danger[x][y+1] == 0 && is_box(free[x][y+1]) {
        return true;
    }
    return false;
}

fn is_box_in_range(x: usize, y: usize, danger: [[i32; 11]; 13], free: [[char; 11]; 13], range: i32) -> bool {
    let mut min32 = x as i32 - (range - 1);
    let mut min = 0;
    if min32 > 0 {
        min = min32 as usize;
    }
    let mut max = x + range as usize;
    if max > 13 {
        max = 13;
    }
    for j in (min..x).rev() {
        if danger[j][y] == 0 && is_box(free[j][y]) {
            // print_err!("box found {} {} -> {} {}", x, y, j, y);
            return true;
        }
        if !is_free(free[j][y]) {
            break;
        }
    }
    for j in x..max {
        if danger[j][y] == 0 && is_box(free[j][y]) {
            // print_err!("box found {} {} -> {} {}", x, y, j, y);
            return true;
        }
        if j != x && !is_free(free[j][y]) {
            break;
        }
    }
    min32 = y as i32 - (range - 1);
    min = 0;
    if min32 > 0 {
        min = min32 as usize;
    }
    max = y + range as usize;
    if max > 11 {
        max = 11;
    }
    for j in (min..y).rev() {
        if danger[x][j] == 0 && is_box(free[x][j]) {
            // print_err!("box found {} {} -> {} {}", x, y, x, j);
            return true;
        }
        if !is_free(free[x][j]) {
            break;
        }
    }
    for j in y..max {
        if danger[x][j] == 0 && is_box(free[x][j]) {
            // print_err!("box found {} {} -> {} {}", x, y, x, j);
            return true;
        }
        if j != y && !is_free(free[x][j]) {
            break;
        }
    }
    return false;
}

fn is_box(c: char) -> bool {
    match c {
        '0' => true,
        '1' => true,
        '2' => true,
        _ => false,
    }
}

fn is_free(c: char) -> bool {
    return c == '.';
}

fn is_walkable(c: char) -> bool {
    match c {
        '.' => true,
        'P' => true,
        _ => false,
    }
}

fn is_step_safe(x: usize, y: usize, danger: [[i32; 11]; 13], free: [[char; 11]; 13], steps: i32) -> bool {
    // print_err!("Is step safe {},{} danger: {} steps: {}", x, y, danger[x][y], steps);
    if !is_walkable(free[x][y]) {
        return false;
    }
    let mut d = danger[x][y];
    while d != 0 {
        if d % 10 == steps {
            return false;
        }
        d = d/10;
    }
    return true;
}
extern crate rand;

use std::io;
use rand::Rng;

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

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let my_ship_count = parse_input!(input_line, i32); // the number of remaining ships
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. ships, mines or cannonballs)
        let mut enemies = vec![];;
        let mut ships = vec![];
        let mut barrels = vec![];
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_id = parse_input!(inputs[0], i32);
            let entity_type = inputs[1].trim().to_string();
            let x = parse_input!(inputs[2], i32);
            let y = parse_input!(inputs[3], i32);
            let arg_1 = parse_input!(inputs[4], i32);
            let arg_2 = parse_input!(inputs[5], i32);
            let arg_3 = parse_input!(inputs[6], i32);
            let arg_4 = parse_input!(inputs[7], i32);
            if entity_type == "BARREL" {
                barrels.push((x, y, arg_1, arg_2));
            } else if entity_type == "SHIP" {
                if arg_4 == 0 {
                    enemies.push((x, y, arg_1, arg_2));
                } else {
                    ships.push((x, y));
                }
            }
        }
        for ship in ships {

            // Write an action using println!("message...");
            // To debug: print_err!("Debug message...");
            // Any valid action, such as "WAIT", FIRE, "MOVE x y"
            let enemy = closest(ship, &enemies);
            let enemy_dist = dist(ship, enemy);
            print_err!("enemy_dist: {}", enemy_dist);
            if enemy_dist < 6 || barrels.is_empty() {
                let mut calculate_pos_dist = 0;
                if enemy_dist != 0 {
                    calculate_pos_dist = enemy_dist - 1;
                }
                let future_pos = calculate_enemy_pos(enemy, calculate_pos_dist);
                println!("FIRE {} {}", future_pos.0, future_pos.1);
            } else {
                let barrel = closest(ship, &barrels);
                println!("MOVE {} {}", barrel.0, barrel.1);
            }
        }
    }
}

fn closest(ship: (i32, i32), entities: &Vec<(i32, i32, i32, i32)>) -> &(i32, i32, i32, i32) {
    let mut result = &entities[0];
    let mut min_dist = dist(ship, &result);
    for entity in entities {
        let entity_dist = dist(ship, &entity);
        if entity_dist < min_dist {
            min_dist = entity_dist;
            result = entity;
        }
    }
    return result;
}

// calculate distance, not exact for hex but good for now ...
fn dist((x1, y1): (i32, i32), &(x2, y2, arg_1, arg_2): &(i32, i32, i32, i32)) -> i32 {
    return (x2-x1).abs() + (y2-y1).abs();
} 

fn calculate_enemy_pos(&(x, y, rotation, speed): &(i32, i32, i32, i32), dist: i32) -> (i32, i32) {
    if rotation == 0 {
        return (x + speed*dist, y);
    } else if rotation == 3 {
        return (x - speed*dist, y);
    } else {
        let iterations = speed*dist;
        print_err!("iterations: {}", iterations);
        let mut new_pos = (x, y);
        for i in 0..iterations {
            // Even...
            if new_pos.1 % 2 == 0 {
                new_pos = match rotation {
                    1 => (new_pos.0, new_pos.1 - 1),
                    2 => (new_pos.0 - 1, new_pos.1 - 1),
                    4 => (new_pos.0 - 1, new_pos.1 + 1),
                    5 => (new_pos.0, new_pos.1 + 1),
                    _ => (new_pos.0, new_pos.1), // How can we say this will never happen ?...
                }
            } else {
                new_pos = match rotation {
                    1 => (new_pos.0 + 1, new_pos.1 - 1),
                    2 => (new_pos.0, new_pos.1 - 1),
                    4 => (new_pos.0, new_pos.1 + 1),
                    5 => (new_pos.0 + 1, new_pos.1 + 1),
                    _ => (new_pos.0, new_pos.1), // How can we say this will never happen ?...
                }
            }
        }
        return new_pos;
    }
}


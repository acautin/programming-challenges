use std::io;

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
 * Shoot enemies before they collect all the incriminating data!
 * The closer you are to an enemy, the more damage you do but don't get too close or you'll get killed.
 **/

 // GOAL < 100.
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let x = parse_input!(inputs[0], i32);
        let y = parse_input!(inputs[1], i32);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let data_count = parse_input!(input_line, i32);
        let mut data_points = vec![];
        for _ in 0..data_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let data_id = parse_input!(inputs[0], i32);
            let data_x = parse_input!(inputs[1], i32);
            let data_y = parse_input!(inputs[2], i32);
            data_points.push((data_id, data_x, data_y));
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let enemy_count = parse_input!(input_line, i32);
        let mut min_dist = i32::max_value();
        let mut target = 0;
        let mut target_x = x;
        let mut target_y = y;
        let mut target_life = 0;

        let mut target_curr_pos = (x, y);

        let mut enemies = vec![];
        for _ in 0..enemy_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let enemy_id = parse_input!(inputs[0], i32);
            let mut enemy_x = parse_input!(inputs[1], i32);
            let mut enemy_y = parse_input!(inputs[2], i32);
            let enemy_life = parse_input!(inputs[3], i32);
            
            let mut data_min_dist = i32::max_value();
            let mut target_data_x = enemy_x;
            let mut target_data_y = enemy_y;
            for &(_, data_x, data_y) in &data_points {
                let data_dist = (data_x - enemy_x).pow(2) + (data_y - enemy_y).pow(2);
                if data_dist < data_min_dist {
                    data_min_dist = data_dist;
                    target_data_x = data_x;
                    target_data_y = data_y;
                }
            }
            let magnitude = (data_min_dist as f64).sqrt() / 500.0;
            let tmp_enemy_pos = (enemy_x, enemy_y);
            if magnitude <= 1.0 {
                enemy_x = target_data_x;
                enemy_y = target_data_y;
            } else {
                let direction = (((target_data_x - enemy_x) as f64)/magnitude, ((target_data_y - enemy_y) as f64)/magnitude);
                enemy_x = enemy_x + (direction.0 as i32);
                enemy_y = enemy_y + (direction.1 as i32);
            }

            let enemy_dist = (enemy_x - x).pow(2) + (enemy_y - y).pow(2);
            if enemy_dist < min_dist {
                min_dist = enemy_dist;
                target = enemy_id;
                target_x = enemy_x;
                target_y = enemy_y;
                target_life = enemy_life;
                target_curr_pos = tmp_enemy_pos;
            }
            enemies.push((enemy_id, enemy_x, enemy_y, enemy_life));
        }

        let mut shoot_dist = 25000000;
        if min_dist > 4000000 {
            shoot_dist = get_shoot_distance((x, y), target_curr_pos, target_life, data_points.as_slice(), enemies.as_slice());
            print_err!("Shoot dist {}", shoot_dist);
        }
        let mut single_move = shoot_or_move((x, y), (target_x, target_y), target_life, enemies.as_slice(), shoot_dist, min_dist);
        let mut max_score = calculate_score(single_move, (x, y), target, data_points.as_slice(), enemies.as_slice(), shoot_dist);
        
        // Test alternative moves, shoots for now
        if enemy_count < 20 {
            for &(enemy_id, enemy_x, enemy_y, _) in &enemies {
                let enemy_dist = (enemy_x - x).pow(2) + (enemy_y - y).pow(2);
                let damage = ((125000.0 / (enemy_dist as f64).powf(0.6)) + 0.5) as i32;
                let score = calculate_score((false, damage, 0), (x, y), enemy_id, data_points.as_slice(), enemies.as_slice(), shoot_dist);
                if score > max_score {
                    single_move = (false, damage, 0);
                    max_score = score;
                    target = enemy_id;
                }
            }
            let score_n = calculate_score((true, x, y - 1000), (x, y), target, data_points.as_slice(), enemies.as_slice(), shoot_dist);
            if score_n > max_score {
                single_move = (true, x, y - 1000);
                max_score = score_n;
            }
            let score_s = calculate_score((true, x, y + 1000), (x, y), target, data_points.as_slice(), enemies.as_slice(), shoot_dist);
            if score_s > max_score {
                single_move = (true, x, y + 1000);
                max_score = score_s;
            }
            let score_e = calculate_score((true, x + 1000, y), (x, y), target, data_points.as_slice(), enemies.as_slice(), shoot_dist);
            if score_e > max_score {
                single_move = (true, x + 1000, y);
                max_score = score_e;
            }
            let score_w = calculate_score((true, x - 1000, y), (x, y), target, data_points.as_slice(), enemies.as_slice(), shoot_dist);
            if score_w > max_score {
                single_move = (true, x - 1000, y);
                max_score = score_w;
            }
        }

        match single_move {
            (true, move_x, move_y) => println!("MOVE {} {}", move_x, move_y),
            _ => println!("SHOOT {}", target),
        }
        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");
        
    }
}

fn calculate_score(mut test_move: (bool, i32, i32), mut current_pos: (i32, i32), mut target_id: i32, data_points: &[(i32, i32, i32)], enemies: &[(i32, i32, i32, i32)], shoot_dist: i32) -> i32 {
    let mut score = 0;
    let mut shoots = 0;
    let total_life = calculate_total_life(enemies);
    let mut tmp_enemies = vec![];
    let mut tmp_data_points = vec![];

    for &(data_id, data_x, data_y) in data_points {
        tmp_data_points.push((data_id, data_x, data_y));
    }

    for &(enemy_id, enemy_x, enemy_y, enemy_life) in enemies {
        let x;
        let y;
        match test_move {
            (true, move_x, move_y) => {
                x = move_x;
                y = move_y;
            },
            _ => {
                x = current_pos.0;
                y = current_pos.1; 
            },
        }
        let enemy_dist = (enemy_x - x).pow(2) + (enemy_y - y).pow(2);
        if enemy_dist < 4000000 {
            return 0;
        }
        tmp_enemies.push((enemy_id, enemy_x, enemy_y, enemy_life));
    }

    loop {
        let mut alive_enemies = vec![];
        match test_move {
            (true, move_x, move_y) => {
                current_pos = (move_x, move_y);
                for &(enemy_id, enemy_x, enemy_y, enemy_life) in &tmp_enemies {
                    alive_enemies.push((enemy_id, enemy_x, enemy_y, enemy_life));
                }
            },
            (false, damage, _) => {
                shoots += 1;
                for &(enemy_id, enemy_x, enemy_y, enemy_life) in &tmp_enemies {
                    if enemy_id != target_id {
                        alive_enemies.push((enemy_id, enemy_x, enemy_y, enemy_life));
                    } else if enemy_life > damage {
                        let new_life = enemy_life - damage;
                        alive_enemies.push((enemy_id, enemy_x, enemy_y, new_life));
                    } else {;
                        score += 10;
                    }
                }
            },
        }        
        let mut alive_data_points = vec![];
        for &(data_id, data_x, data_y) in &tmp_data_points {
            let mut found = false;
            for &(_, enemy_x, enemy_y, _) in &alive_enemies {
                if data_x == enemy_x && data_y == enemy_y {
                    found = true;
                    break;
                }
            }
            if !found {
                alive_data_points.push((data_id, data_x, data_y));
            }
        }
        let data_count = alive_data_points.len() as i32;
        if alive_data_points.len() == 0 || alive_enemies.len() == 0 {
            score += data_count * 100;
            let bonus = data_count * (total_life - 3 * shoots) * 3;
            if bonus > 0 {
                score += bonus;
            }
            break;
        }

        tmp_enemies.clear();
        let mut min_dist = i32::max_value();
        let mut target_x = current_pos.0;
        let mut target_y = current_pos.1;
        let mut target_life = 0;
        for &(enemy_id, mut enemy_x, mut enemy_y, enemy_life) in &alive_enemies {
            let mut data_min_dist = i32::max_value();
            let mut target_data_x = enemy_x;
            let mut target_data_y = enemy_y;
            for &(_, data_x, data_y) in &alive_data_points {
                let data_dist = (data_x - enemy_x).pow(2) + (data_y - enemy_y).pow(2);
                if data_dist < data_min_dist {
                    data_min_dist = data_dist;
                    target_data_x = data_x;
                    target_data_y = data_y;
                }
            }
            let magnitude = (data_min_dist as f64).sqrt() / 500.0;
            if magnitude <= 1.0 {
                enemy_x = target_data_x;
                enemy_y = target_data_y;
            } else {
                let direction = (((target_data_x - enemy_x) as f64)/magnitude, ((target_data_y - enemy_y) as f64)/magnitude);
                enemy_x = enemy_x + (direction.0 as i32);
                enemy_y = enemy_y + (direction.1 as i32);
            }

            let enemy_dist = (enemy_x - current_pos.0).pow(2) + (enemy_y - current_pos.1).pow(2);
            if enemy_dist < min_dist {
                min_dist = enemy_dist;
                target_id = enemy_id;
                target_x = enemy_x;
                target_y = enemy_y;
                target_life = enemy_life;
            }
            tmp_enemies.push((enemy_id, enemy_x, enemy_y, enemy_life));
        }
        test_move = shoot_or_move(current_pos, (target_x, target_y), target_life, tmp_enemies.as_slice(), shoot_dist, min_dist);
        tmp_data_points.clear();
        for &(data_id, data_x, data_y) in &alive_data_points {
            tmp_data_points.push((data_id, data_x, data_y));
        }
    }
    return score;
}

fn get_shoot_distance(current_pos: (i32, i32), enemy_pos: (i32, i32), enemy_life: i32, data_points: &[(i32, i32, i32)], enemies: &[(i32, i32, i32, i32)]) -> i32 {
    let mut result_dist = 25000000;
    let mut test_dist = 25000000;
    let mut min_shoots = i32::max_value(); 
    while test_dist >= 9000000 {
        //TODO: Extract this to a function
        let mut data_min_dist = i32::max_value();
        let mut target_pos = enemy_pos;
        for &(_, data_x, data_y) in data_points {
            let data_dist = (data_x - enemy_pos.0).pow(2) + (data_y - enemy_pos.1).pow(2);
            if data_dist < data_min_dist {
                data_min_dist = data_dist;
                target_pos = (data_x, data_y);
            }
        }
        let mut test_my_pos = current_pos;
        let mut test_enemy_pos = enemy_pos;
        let mut test_enemy_life = enemy_life;
        let mut shoots = 0;
        let mut game_over = false;
        while !game_over {
            let data_dist = (target_pos.0 - test_enemy_pos.0).pow(2) + (target_pos.1 - test_enemy_pos.1).pow(2);
            let magnitude = (data_dist as f64).sqrt() / 500.0;
            if magnitude <= 1.0 {
                // Test for the kill...
                let enemy_dist = (target_pos.0 - test_my_pos.0).pow(2) + (target_pos.1 - test_my_pos.1).pow(2);
                let damage = ((125000.0 / (enemy_dist as f64).powf(0.6)) + 0.5) as i32;
                if damage < test_enemy_life {
                    shoots = i32::max_value();
                } else {
                    shoots = shoots + 1;
                }
                game_over = true;
            } else {
                let direction = (((target_pos.0 - test_enemy_pos.0) as f64)/magnitude, ((target_pos.1 - test_enemy_pos.1) as f64)/magnitude);
                test_enemy_pos = (test_enemy_pos.0 + (direction.0 as i32), test_enemy_pos.1 + (direction.1 as i32));
                let enemy_dist = (test_enemy_pos.0 - test_my_pos.0).pow(2) + (test_enemy_pos.1 - test_my_pos.1).pow(2);

                let single_move = shoot_or_move(test_my_pos, test_enemy_pos, test_enemy_life, enemies, test_dist, enemy_dist);

                match single_move {
                    (true, move_x, move_y) => test_my_pos = (move_x, move_y),
                    (false, damage, _) => {
                        shoots = shoots + 1;
                        test_enemy_life = test_enemy_life - damage;
                    },
                }
                if test_enemy_life <= 0 {
                    game_over = true;
                }
            }
        }
        if shoots <= min_shoots {
            min_shoots = shoots;
            result_dist = test_dist;
        }
        test_dist = test_dist - 1000000;
    }
    return result_dist;
}

fn shoot_or_move(current_pos: (i32, i32), target_pos: (i32, i32), enemy_life: i32, enemies: &[(i32, i32, i32, i32)], shoot_dist: i32, min_dist: i32) -> (bool, i32, i32) {
    let damage = ((125000.0 / (min_dist as f64).powf(0.6)) + 0.5) as i32;    
    if min_dist > shoot_dist {
        if damage < enemy_life {
            let magnitude = (min_dist as f64).sqrt() / 1000.0;
            let direction = (((target_pos.0 - current_pos.0) as f64)/magnitude, ((target_pos.1 - current_pos.1) as f64)/magnitude);
            let target_x = current_pos.0 + (direction.0 as i32);
            let target_y = current_pos.1 + (direction.1 as i32);
            return (true, target_x, target_y);
        } else {
            return (false, damage, 0);
        }
    } else if min_dist > 4000000 {
        return (false, damage, 0);
    } else {
        let mut magnitude = (min_dist as f64).sqrt() / 1000.0;
        let mut direction = (((current_pos.0 - target_pos.0) as f64)/magnitude, ((current_pos.1 - target_pos.1) as f64)/magnitude);
        let mut target_x = current_pos.0 + (direction.0 as i32);
        let mut target_y = current_pos.1 + (direction.1 as i32);
        for &(_, enemy_x, enemy_y, _) in enemies {
            let enemy_dist = (enemy_x - target_x).pow(2) + (enemy_y - target_y).pow(2);
            if enemy_dist < 4000000 {
                magnitude = (enemy_dist as f64).sqrt() / 500.0;
                direction = (((target_x - enemy_x) as f64)/magnitude, ((target_y - enemy_y) as f64)/magnitude);
                target_x = target_x + (direction.0 as i32);
                target_y = target_y + (direction.1 as i32);
            }
        }
        return (true, target_x, target_y);
    }
}

fn calculate_total_life(enemies: &[(i32, i32, i32, i32)]) -> i32 {
    let mut total_life = 0;
    for &(_, _, _, life) in enemies {
        total_life += life;
    }
    return total_life;
}

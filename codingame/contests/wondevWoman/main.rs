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
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let size = parse_input!(input_line, i32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let units_per_player = parse_input!(input_line, i32);

    // game loop
    loop {
        let mut grid = vec![];
        let mut my_units = vec![];
        for i in 0..size as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let row = input_line.trim().to_string();
            grid.push(row);
        }
        for i in 0..units_per_player as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let unit_x = parse_input!(inputs[0], i32);
            let unit_y = parse_input!(inputs[1], i32);
            my_units.push((unit_x, unit_y));
        }
        for i in 0..units_per_player as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let other_x = parse_input!(inputs[0], i32);
            let other_y = parse_input!(inputs[1], i32);
        }

        let mut actions = vec![];

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let legal_actions = parse_input!(input_line, i32);
        for i in 0..legal_actions as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let atype = inputs[0].trim().to_string();
            let index = parse_input!(inputs[1], i32);
            let dir_1 = inputs[2].trim().to_string();
            let dir_2 = inputs[3].trim().to_string();

            let move_from = my_units[index as usize];
            let move_to = get_pos(&move_from, &dir_1);
            let to_height = get_height(&move_to, grid.as_slice());
            let from_height = get_height(&move_from, grid.as_slice());
            let build_pos = get_pos(&move_to, &dir_2);
            let build_height = get_height(&build_pos, grid.as_slice());
            actions.push((atype, index, dir_1, dir_2, from_height, to_height, build_height, move_from, move_to));
        }

        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");
        print_grid(&grid);
        print_action(best_action(actions.as_slice()));
        // println!("MOVE&BUILD 0 N S");
    }
}

fn print_grid(rows: &[String]) {
    for row in rows {
        print_err!("{}", row);
    }
}

fn print_action(action: &(String, i32, String, String, u32, u32, u32, (i32, i32), (i32, i32))) {
    print_err!("printing action {:?}", action);
    println!("{} {} {} {}", action.0, action.1, action.2, action.3);
}

fn get_pos(from: &(i32, i32), dir: &String) -> (i32, i32) {
    let mut x = from.0;
    let mut y = from.1;
    if dir.contains("N") {
        y -= 1;
    } else if dir.contains("S") {
        y += 1;
    }

    if dir.contains("W") {
        x -= 1;
    } else if dir.contains("E") {
        x += 1;
    }

    return (x, y);
}

fn get_height(pos: &(i32, i32), grid: &[String]) -> u32 {
    return grid[pos.1 as usize].chars().nth(pos.0 as usize).unwrap().to_digit(10).unwrap();
}

fn best_action(actions: &[(String, i32, String, String, u32, u32, u32, (i32, i32), (i32, i32))]) -> &(String, i32, String, String, u32, u32, u32, (i32, i32), (i32, i32)) {
    let mut selected = &actions[0];
    let mut max_height = selected.5;
    let mut min_build = selected.6;
    for action in actions {
        if action.5 > max_height {
            selected = action;
            max_height = action.5;
            min_build = action.6;
        } else if action.5 == max_height && action.6 < min_build {
            selected = action;
            max_height = action.5;
            min_build = action.6;
        }
    }
    return selected;
}

use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn r() -> Vec<String> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.split(" ").map(|s| s.to_string()).collect()
}

fn main() {
    let inputs = r();
    let exit_floor = parse_input!(inputs[3], i32);
    let exit_pos = parse_input!(inputs[4], i32);
    let nb_elevators = parse_input!(inputs[7], i32);
    let mut elevator_p = [0;100];
    for i in 0..nb_elevators as usize {
        let inputs = r();
        elevator_p[parse_input!(inputs[0], usize)] = parse_input!(inputs[1], i32);
    }

    let mut current_floor = 0;
    loop {
        let inputs = r();
        let clone_floor = parse_input!(inputs[0], i32);
        let clone_pos = parse_input!(inputs[1], i32);
        let direction = inputs[2].trim().to_string();
        let mut cmd = "WAIT";
        if clone_floor == current_floor {
            let target_pos = if exit_floor == clone_floor { exit_pos } else { elevator_p[clone_floor as usize] };
            if direction == "RIGHT" && clone_pos > target_pos || direction == "LEFT" && clone_pos < target_pos {
                cmd = "BLOCK";
            }
            current_floor += 1;
        }
        println!("{}", cmd);
    }
}

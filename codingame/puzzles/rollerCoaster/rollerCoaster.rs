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

#[derive(Copy, Clone)]
struct PrecalcInfo {
    next: usize,  // Starting position for the next ride 
    count: i64      // Accumulated passengers (earnings)
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    print_err!("the input { }", input_line);
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let l = parse_input!(inputs[0], i32); // Places in the Roller Coaster
    let c = parse_input!(inputs[1], i32); // Number of rides in a day
    let n = parse_input!(inputs[2], usize); // length of the group queue.
    
    let mut groups = [0; 10000];  // Restriction of the problem n <= 10000
    let mut precalculated = [PrecalcInfo{next: 10000, count: 0}; 10000];
    
    for i in 0..n {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        groups[i] = parse_input!(input_line, i32);
    }
    
    let mut result: i64 = 0;
    let mut current_group = 0;
        
    for _ in 0..c {
        let mut capacity_left = l;
        if precalculated[current_group].next == 10000 {
            let starting_group = current_group;
            let mut count: i64 = 0;
            while capacity_left - groups[current_group] >= 0 {
                count += groups[current_group] as i64;
                capacity_left -= groups[current_group];
                current_group += 1;
                if current_group == n {
                    current_group = 0;
                }
                if current_group == starting_group {
                    break; 
                }
            }
            precalculated[starting_group].count = count;
            precalculated[starting_group].next = current_group;
            result += count;
        } else {
            result += precalculated[current_group].count;
            current_group = precalculated[current_group].next;
        }
    }
    println!("{ }", result);
}

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
 * Bring data on patient samples from the diagnosis machine to the laboratory with enough molecules to produce medicine!
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let project_count = parse_input!(input_line, i32);
    for i in 0..project_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let a = parse_input!(inputs[0], i32);
        let b = parse_input!(inputs[1], i32);
        let c = parse_input!(inputs[2], i32);
        let d = parse_input!(inputs[3], i32);
        let e = parse_input!(inputs[4], i32);
    }

    // game loop
    loop {
        let mut target = "".to_string();
        let mut samples = vec![];
        let mut my_storage = (0, 0, 0, 0, 0);
        let mut my_samples = 0;
        let mut sample_1 = (-1, -2, 0, "".to_string(), -5, 0, 0, 0, 0, 0);
        let mut sample_2 = (-1, -2, 0, "".to_string(), -5, 0, 0, 0, 0, 0);
        let mut sample_3 = (-1, -2, 0, "".to_string(), -5, 0, 0, 0, 0, 0);
        let mut my_expertise = (0, 0, 0, 0, 0);
        for i in 0..2 as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let eta = parse_input!(inputs[1], i32);
            let score = parse_input!(inputs[2], i32);
            let storage_a = parse_input!(inputs[3], i32);
            let storage_b = parse_input!(inputs[4], i32);
            let storage_c = parse_input!(inputs[5], i32);
            let storage_d = parse_input!(inputs[6], i32);
            let storage_e = parse_input!(inputs[7], i32);
            let expertise_a = parse_input!(inputs[8], i32);
            let expertise_b = parse_input!(inputs[9], i32);
            let expertise_c = parse_input!(inputs[10], i32);
            let expertise_d = parse_input!(inputs[11], i32);
            let expertise_e = parse_input!(inputs[12], i32);
            if i == 0 {
                my_storage = (storage_a, storage_b, storage_c, storage_d, storage_e);
                my_expertise = (expertise_a, expertise_b, expertise_c, expertise_d, expertise_e);
                target = inputs[0].trim().to_string();
            }
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let available_a = parse_input!(inputs[0], i32);
        let available_b = parse_input!(inputs[1], i32);
        let available_c = parse_input!(inputs[2], i32);
        let available_d = parse_input!(inputs[3], i32);
        let available_e = parse_input!(inputs[4], i32);
        let available = (available_a, available_b, available_c, available_d, available_e);
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let sample_count = parse_input!(input_line, i32);
        for i in 0..sample_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let sample_id = parse_input!(inputs[0], i32);
            let carried_by = parse_input!(inputs[1], i32);
            let rank = parse_input!(inputs[2], i32);
            let expertise_gain = inputs[3].trim().to_string();
            let health = parse_input!(inputs[4], i32);
            let cost_a = parse_input!(inputs[5], i32);
            let cost_b = parse_input!(inputs[6], i32);
            let cost_c = parse_input!(inputs[7], i32);
            let cost_d = parse_input!(inputs[8], i32);
            let cost_e = parse_input!(inputs[9], i32);
            samples.push((sample_id, carried_by, rank, expertise_gain, health, cost_a, cost_b, cost_c, cost_d, cost_e));
            if carried_by == 0 {
                if my_samples == 0 {
                    sample_1 = (sample_id, carried_by, rank, "".to_string(), health, cost_a, cost_b, cost_c, cost_d, cost_e);
                } else if my_samples == 1 {
                    sample_2 = (sample_id, carried_by, rank, "".to_string(), health, cost_a, cost_b, cost_c, cost_d, cost_e);
                } else {
                    sample_3 = (sample_id, carried_by, rank, "".to_string(), health, cost_a, cost_b, cost_c, cost_d, cost_e);
                }
                my_samples += 1;
            }
        }

        let rank = select_rank(my_expertise);
        let cost_1 = (sample_1.5, sample_1.6, sample_1.7, sample_1.8, sample_1.9);
        let cost_2 = (sample_2.5, sample_2.6, sample_2.7, sample_2.8, sample_2.9);
        let cost_3 = (sample_3.5, sample_3.6, sample_3.7, sample_3.8, sample_3.9);
        let mol_1 = needed_molecule(my_storage, my_expertise, cost_1);
        let mol_2 = needed_molecule(my_storage, my_expertise, cost_2);
        let mol_3 = needed_molecule(my_storage, my_expertise, cost_3);
        let extra_mol_1 = extra_molecule(my_storage, my_expertise, cost_1, my_samples, cost_2, cost_3, available);
        let extra_mol_2 = extra_molecule(my_storage, my_expertise, cost_2, my_samples, cost_1, cost_3, available);
        let extra_mol_3 = extra_molecule(my_storage, my_expertise, cost_3, my_samples, cost_1, cost_2, available);
        print_err!("exp {} {} {} {} {}", my_expertise.0, my_expertise.1, my_expertise.2, my_expertise.3, my_expertise.4);
        print_err!("cost {} {} {} {} {}", cost_1.0, cost_1.1, cost_1.2, cost_1.3, cost_1.4);
        print_err!("cost {} {} {} {} {}", cost_2.0, cost_2.1, cost_2.2, cost_2.3, cost_2.4);
        print_err!("cost {} {} {} {} {}", cost_3.0, cost_3.1, cost_3.2, cost_3.3, cost_3.4);

        print_err!("extra_mol 1 {}", extra_mol_1);
        print_err!("extra mol 2 {}", extra_mol_2);
        print_err!("extra mol 3 {}", extra_mol_3);
        if my_samples == 0 {
            goto_and_connect("SAMPLES".to_string(), target, rank, "none".to_string());
        } else if my_samples < 3 && target == "SAMPLES" {
            println!("CONNECT {}", rank);
        } else if sample_1.5 < 0 {
            goto_and_connect("DIAGNOSIS".to_string(), target, sample_1.0, "none".to_string());
        } else if my_samples > 1 && sample_2.5 < 0 {
            goto_and_connect("DIAGNOSIS".to_string(), target, sample_2.0, "none".to_string());
        } else if my_samples > 2 && sample_3.5 < 0 {
            goto_and_connect("DIAGNOSIS".to_string(), target, sample_3.0, "none".to_string());
        } else if imposible(my_expertise, cost_1) && target == "DIAGNOSIS"  {
            println!("CONNECT {}", sample_1.0);
        } else if my_samples > 1 && imposible(my_expertise, cost_2) && target == "DIAGNOSIS" {
            println!("CONNECT {}", sample_2.0);
        } else if my_samples > 2 && imposible(my_expertise, cost_3) && target == "DIAGNOSIS" {
            println!("CONNECT {}", sample_3.0);
        } else {
            if extra_mol_1 != "none" && enough_available_molecules(&extra_mol_1, available) && free_space(my_storage) && (target == "MOLECULES" || target == "DIAGNOSIS") {
                goto_and_connect("MOLECULES".to_string(), target, sample_1.0, extra_mol_1);
            } else if my_samples > 1 && extra_mol_2 != "none" && enough_available_molecules(&extra_mol_2, available) && free_space(my_storage) && (target == "MOLECULES" || target == "DIAGNOSIS") {
                goto_and_connect("MOLECULES".to_string(), target, sample_2.0, extra_mol_2);
            } else if my_samples > 2 && extra_mol_3 != "none" && enough_available_molecules(&extra_mol_3, available) && free_space(my_storage) && (target == "MOLECULES" || target == "DIAGNOSIS") {
                goto_and_connect("MOLECULES".to_string(), target, sample_3.0, extra_mol_3);
            } else if mol_1 == "none" {
                goto_and_connect("LABORATORY".to_string(), target, sample_1.0, "none".to_string());
            } else if my_samples > 1 && mol_2 == "none" {
                goto_and_connect("LABORATORY".to_string(), target, sample_2.0, "none".to_string());
            } else if my_samples > 2 && mol_3 == "none" {
                goto_and_connect("LABORATORY".to_string(), target, sample_3.0, "none".to_string());
            } else if enough_available_molecules(&mol_1, available) && free_space(my_storage) {
                goto_and_connect("MOLECULES".to_string(), target, sample_1.0, mol_1);
            } else if my_samples > 1 && enough_available_molecules(&mol_2, available) && free_space(my_storage) {
                goto_and_connect("MOLECULES".to_string(), target, sample_2.0, mol_2);
            } else if my_samples > 2 && enough_available_molecules(&mol_3, available)  && free_space(my_storage) {
                goto_and_connect("MOLECULES".to_string(), target, sample_3.0, mol_3);
            } else if my_samples < 3 {
                goto_and_connect("SAMPLES".to_string(), target, rank, "none".to_string());
            } else {
                /*
                if free_space(my_storage) {
                    if enough_available_molecules("A", available) {
                        goto_and_connect("MOLECULES".to_string(), target, sample_1.0, "A".to_string());
                    } else if enough_available_molecules("B", available) {
                        goto_and_connect("MOLECULES".to_string(), target, sample_1.0, "B".to_string());
                    } else if enough_available_molecules("C", available) {
                        goto_and_connect("MOLECULES".to_string(), target, sample_1.0, "C".to_string());
                    } else if enough_available_molecules("D", available) {
                        goto_and_connect("MOLECULES".to_string(), target, sample_1.0, "D".to_string());
                    } else if enough_available_molecules("E", available) {
                        goto_and_connect("MOLECULES".to_string(), target, sample_1.0, "E".to_string());
                    } else {
                        println!("WAIT");
                    }
                } else {
                    println!("WAIT");
                }
                */
                println!("WAIT");
            }
        }
    }
}

fn goto_and_connect(module: String, position: String, data_id: i32, molecule: String) {
    if module == position {
        if module == "MOLECULES" {
            println!("CONNECT {}", molecule);
        } else {
            println!("CONNECT {}", data_id);
        }
    } else {
        println!("GOTO {}", module);
    }
}

fn needed_molecule(storage: (i32, i32, i32, i32, i32), expertise: (i32, i32, i32, i32, i32), cost: (i32, i32, i32, i32, i32)) -> String {
    if storage.0 < cost.0 - expertise.0 {
        return "A".to_string();
    }
    if storage.1 < cost.1 - expertise.1 {
        return "B".to_string();
    }
    if storage.2 < cost.2 - expertise.2 {
        return "C".to_string();
    }
    if storage.3 < cost.3 - expertise.3 {
        return "D".to_string();
    }
    if storage.4 < cost.4 - expertise.4 {
        return "E".to_string();
    }
    return "none".to_string();
}

fn extra_molecule(storage: (i32, i32, i32, i32, i32), expertise: (i32, i32, i32, i32, i32), cost: (i32, i32, i32, i32, i32), count: i32, cost_2: (i32, i32, i32, i32, i32), cost_1: (i32, i32, i32, i32, i32), available: (i32, i32, i32, i32, i32)) -> String {
    if available.0 > 0 && storage.0 < cost.0 + cost_1.0 + cost_2.0 - expertise.0 {
        return "A".to_string();
    }
    if available.1 > 0 && storage.1 < cost.1 + cost_1.1 + cost_2.1 - expertise.1 {
        return "B".to_string();
    }
    if available.2 > 0 && storage.2 < cost.2 + cost_1.2 + cost_2.2 - expertise.2 {
        return "C".to_string();
    }
    if available.3 > 0 && storage.3 < cost.3 + cost_1.3 + cost_2.3 - expertise.3 {
        return "D".to_string();
    }
    if available.4 > 0 && storage.4 < cost.4 + cost_1.4 + cost_2.4 - expertise.4 {
        return "E".to_string();
    }
    return "none".to_string();
}

fn enough_available_molecules(mol: &str, available: (i32, i32, i32, i32, i32)) -> bool {
    match mol {
        "A" => available.0 > 0,
        "B" => available.1 > 0,
        "C" => available.2 > 0,
        "D" => available.3 > 0,
        "E" => available.4 > 0,
        _ => true,
    }
}

fn select_rank((a, b, c, d, e): (i32, i32, i32, i32, i32)) -> i32 {
    if a+b+c+d+e > 12 {
        return 3;
    } else if a+b+c+d+e > 9 {
        return 2;
    }
    return 1;
}

fn imposible(expertise: (i32, i32, i32, i32, i32), cost: (i32, i32, i32, i32, i32)) -> bool {
    if cost.0 - expertise.0 > 5 {
        return true;
    }
    if cost.1 - expertise.1 > 5 {
        return true;
    }
    if cost.2 - expertise.2 > 5 {
        return true;
    }
    if cost.3 - expertise.3 > 5 {
        return true;
    }
    if cost.4 - expertise.4 > 5 {
        return true;
    }
    return false;
}

fn free_space((a, b, c, d, e): (i32, i32, i32, i32, i32)) -> bool {
    return a+b+c+d+e < 10;
}

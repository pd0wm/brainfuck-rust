use std::env;
use std::fs;
use std::collections::HashMap;

fn find_matching_closing(contents : &Vec<char>, start : usize) -> usize{
    let mut ip = start;
    let mut depth = 1;
    loop {
        ip += 1;
        let c = contents[ip];
        match c {
            '[' => {
                depth += 1;
            }
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return ip;
                }
            }
            _  => {
            }
        }
    }
}

fn find_matching_opening(contents : &Vec<char>, start : usize) -> usize{
    let mut ip = start;
    let mut depth = 1;
    loop {
        ip -= 1;
        let c = contents[ip];
        match c {
            ']' => {
                depth += 1;
            }
            '[' => {
                depth -= 1;
                if depth == 0 {
                    return ip;
                }
            }
            _  => {
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename : String;

    match args.len() {
        2 => {
            filename = args[1].clone();
        }
        _ => {
            println!("Usage: <filename>");
            return;
        }
    }

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let contents : Vec<char> = contents.chars().collect();

    let mut ip : usize = 0;
    let mut ptr : i64 = 0;
    let mut dat : HashMap<i64, u8> = HashMap::new();
    let mut jump_cache : HashMap<usize, usize> = HashMap::new();

    loop {
        let entry = dat.entry(ptr).or_insert(0);
        let c = contents[ip];

        match c {
            '>' => {
                ptr += 1;
            }
            '<' => {
                ptr -= 1;
            }
            '+' => {
                *entry = entry.wrapping_add(1);
            }
            '-' => {
                *entry = entry.wrapping_sub(1);
            }
            '.' => {
                let entry = char::from(*entry);
                print!("{}", entry);
            }
            ',' => {
                panic!("Input not implemented yet");
            }
            '[' => {
                if *entry == 0 {
                    match jump_cache.get(&ip){
                        Some(v) => {
                            ip = *v;
                        }
                        None => {
                            let target = find_matching_closing(&contents, ip);
                            jump_cache.insert(ip, target);
                            ip = target;
                        }
                    }
                }
            }
            ']' => {
                if *entry != 0 {
                    match jump_cache.get(&ip){
                        Some(v) => {
                            ip = *v;
                        }
                        None => {
                            let target = find_matching_opening(&contents, ip);
                            jump_cache.insert(ip, target);
                            ip = target;
                        }
                    }
                }
            }
            _ => {
            }
        }

        ip += 1;

        if ip >= contents.len() {
            break;
        }
    }

}

use std::collections::HashSet;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i32 {
    let data = load_data();
    let result = test_program(&data);
    result.0
}

fn solution_2() -> i32 {
    let mut data = load_data();
    for i in 0..data.len() {
        let data_line = data[i].to_owned();
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        match parts[0] {
            "acc" => continue,
            "jmp" => {
                let new_line = format!("{} {}", "nop", parts[1]);
                data[i] = new_line;
            },
            "nop" => {
                let new_line = format!("{} {}", "jmp", parts[1]);
                data[i] = new_line;
            },
            _ => unreachable!(),
        }
        let result = test_program(&data);
        data[i] = data_line.to_string();
        if result.1 {
            return result.0;
        }
    }
    0
}

fn test_program(data: &Vec<String>) -> (i32, bool) {
    let mut acc = 0;
    let mut current_line = 0;
    let mut lines = HashSet::new();
    while !lines.contains(&current_line) && current_line < data.len() {
        lines.insert(current_line);
        let parts: Vec<&str> = data[current_line].split_whitespace().collect();
        let value: i32 = parts[1].parse().unwrap_or(0);
        match parts[0] {
            "acc" => {
                acc += value;
                current_line += 1;
            }
            "jmp" => {
                current_line = (current_line as i32 + value) as usize;
            }
            "nop" => {
                current_line += 1;
            }
            _ => unreachable!(),
        }
    }
    (acc, current_line == data.len())
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

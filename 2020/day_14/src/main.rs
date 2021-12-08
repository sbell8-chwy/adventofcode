use std::collections::HashMap;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u64 {
    let data = load_data();
    let mut set_bits = 0u64;
    let mut unset_bits = 0u64;
    let mut memory = HashMap::new();
    data.iter()
        .map(|s| {
            let row_vals: Vec<&str> = s.split(" = ").collect();
            match row_vals[0] {
                "mask" => {
                    let temp = parse_mask(row_vals[1]);
                    set_bits = temp.0;
                    unset_bits = temp.1;
                },
                _ => {
                    let mut value: u64 = row_vals[1].parse().unwrap_or(0);
                    value |= set_bits;
                    value &= unset_bits;
                    memory.insert(row_vals[0], value);
                },
            }
        })
        .count();
    let mut sum = 0u64;
    for (_, v) in memory.iter() {
        sum += v;
    }
    sum
}

fn solution_2() -> u64 {
    let data = load_data();
    let mut memory = HashMap::new();
    let mut current_mask = "";
    data.iter()
        .map(|s| {
            let row_vals: Vec<&str> = s.split(" = ").collect();
            match row_vals[0] {
                "mask" => current_mask = row_vals[1],
                _ => {
                    let registers = get_registers(current_mask, row_vals[0]);
                    for r in registers.into_iter() {
                        memory.insert(r, row_vals[1].parse().unwrap_or(0));
                    }
                },
            }
        })
        .count();
    let mut sum = 0u64;
    for (_, v) in memory.iter() {
        sum += v;
    }
    sum
}

fn get_registers(mask: &str, register: &str) -> Vec<u64> {
    let mut register = register[4..register.len()-1].parse::<u64>().unwrap_or(0);
    let set_bits = parse_mask(mask).0;
    register |= set_bits;
    let mut registers = vec![];
    registers.push(register);
    for (i, c) in mask.chars().enumerate() {
        if c == 'X' {
            let mut temp_registers = vec![];
            let set_bit = 1 << (35-i);
            let unset_bit = !set_bit;
            for r in registers {
                temp_registers.push(r | set_bit);
                temp_registers.push(r & unset_bit);
            }
            registers = temp_registers;
        }
    }
    registers
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut set_bits = 0u64;
    let mut unset_bits = 0u64;

    for c in mask.chars() {
        set_bits <<= 1;
        unset_bits <<= 1;
        match c {
            'X' => unset_bits += 1,
            '1' => {
                set_bits += 1;
                unset_bits += 1;
            },
            '0' => (),
            _ => unreachable!(),
        }
    }

    (set_bits, unset_bits)
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

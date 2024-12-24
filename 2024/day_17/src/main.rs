use std::env;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> String {
    let (mut computer, operations) = load_data();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        computer.register_a = usize::from_str_radix(&args[1], 8).unwrap();
    }
    let output = run_program(&mut computer, &operations);
    println!("Solution 1 => 2,4,1,3,7,5,4,2,0,3,1,5,5,5,3,0");
    output
}

fn solution_2() -> usize {
    let (mut computer, operations) = load_data();
    let program_string = operations.iter()
        .map(|i| {
            let mut temp = i.to_string();
            temp.push_str(",");
            temp
        }).collect::<String>();
    // let mut register_a = 0o6562457404257155;
    let program_digits = [2, 4, 1, 3, 7, 5, 4, 2, 0, 3, 1, 5, 5, 5, 3, 0];
    let mut register_a = 0o6000000000000000;
    let mut depth = 1;
    computer.register_a = register_a;
    loop {
        reset_computer(&mut computer);
        computer.register_a = register_a;
        let output = run_program(&mut computer, &operations);
        if output == program_string {
            break;
        }
        // println!("out: {}", output);
        // println!("pro: {}", program_string);
        let mut output_chars = output.chars();
        output_chars.next_back();
        let output = output_chars.as_str();
        let output_digits: Vec<usize> = output.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        let mut octal_digit = get_octal_digit(&register_a, 16-depth);
        // println!("Checking digit {} against output {}", program_digits[15-depth], output_digits[15-depth]);
        if program_digits[15-depth] != output_digits[15-depth] {
            register_a = increment_octal_digit(&register_a, 16-depth);
            while octal_digit == 7 {
                depth -= 1;
                octal_digit = get_octal_digit(&register_a, 16-depth);
                register_a = increment_octal_digit(&register_a, 16-depth);
            }
            // println!("Incrementing digit {}", depth);
        } else {
            depth += 1;
        }
        println!("Register a: {:o}", register_a);
        // if depth == 5 {
            // break;
        // }
        // register_a += 1;
    }
    register_a
}

fn get_octal_digit(num: &usize, digit: usize) -> usize {
    let shift = (digit -1) * 3;
    (num >> shift) % 8
}

fn increment_octal_digit(num: &usize, digit: usize) -> usize {
    let shift = (digit -1) * 3;
    if (num >> shift) % 8 == 7 {
        num - (7 << shift)
    } else {
        num + (1 << shift)
    }
}

fn reset_computer(computer: &mut ThreeBitComputer) {
    computer.register_a = 0;
    computer.register_b = 0;
    computer.register_c = 0;
}

fn run_program(computer: &mut ThreeBitComputer, operations: &Vec<usize>) -> String {
    let mut instruction_index = 0;
    let mut output = "".to_owned();
    while instruction_index < operations.len() {
        let (jump, out) = process_opcode(computer, operations[instruction_index], operations[instruction_index + 1]);
        match jump {
            Some(x) => instruction_index = x,
            None => instruction_index += 2,
        }
        match out {
            Some(x) => output.push_str(&x),
            None => (),
        }
    }
    output
}

fn process_opcode(computer: &mut ThreeBitComputer, opcode: usize, operand: usize) -> (Option<usize>, Option<String>) {
    match opcode {
        0 => {
            let value = computer.register_a / (2usize.pow(get_combo_operand(&computer, operand) as u32));
            // println!("a = {:o}", value);
            computer.register_a = value;
            (Option::None, Option::None)
        },
        1 => {
            let value = computer.register_b ^ operand;
            // println!("b = {:o}", value);
            computer.register_b = value;
            (Option::None, Option::None)
        },
        2 => {
            let value = get_combo_operand(&computer, operand) % 8;
            // println!("b = {:o}", value);
            computer.register_b = value;
            (Option::None, Option::None)
        },
        3 => {
            if computer.register_a == 0 {
                (Option::None, Option::None)
            } else {
                (Some(operand), Option::None)
            }
        },
        4 => {
            let value = computer.register_b ^ computer.register_c;
            // println!("b = {:b}", value);
            computer.register_b = value;
            (Option::None, Option::None)
        },
        5 => {
            let value = get_combo_operand(&computer, operand) % 8;
            let mut output = value.to_string();
            output.push_str(",");
            (Option::None, Some(output))
        },
        6 => {
            let value = computer.register_a / (2usize.pow(get_combo_operand(&computer, operand) as u32));
            // println!("b = {:o}", value);
            computer.register_b = value;
            (Option::None, Option::None)
        },
        7 => {
            let value = computer.register_a / (2usize.pow(get_combo_operand(&computer, operand) as u32));
            // println!("a = {:b}", computer.register_a);
            // println!("c = {:b}", value);
            computer.register_c = value;
            (Option::None, Option::None)
        },
        _ => (Option::None, Option::None),
    }
}

fn get_combo_operand(computer: &ThreeBitComputer, operand: usize) -> usize {
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => computer.register_a,
        5 => computer.register_b,
        6 => computer.register_c,
        _ => panic!("Invalid Combo Operand"),
    }
}

fn load_data() -> (ThreeBitComputer, Vec<usize>) {
    let mut lines = include_str!("../assets/input.txt")
        .lines();
    let computer = ThreeBitComputer::new(
        lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1].parse().unwrap(),
        lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1].parse().unwrap(),
        lines.next().unwrap().split(": ").collect::<Vec<&str>>()[1].parse().unwrap(),
    );
    lines.next();
    let program_str = lines.next().unwrap().split(": ").collect::<Vec<&str>>();
    let program = program_str[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    (computer, program)
}

struct ThreeBitComputer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
}

impl ThreeBitComputer {
    fn new(a: usize, b: usize, c: usize) -> Self {
        ThreeBitComputer {
            register_a: a,
            register_b: b,
            register_c: c,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_octal_digit_1() {
        let digit = increment_octal_digit(&0o12345usize, 3);
        assert_eq!(digit, 0o12445);
    }

    #[test]
    fn increment_octal_digit_2() {
        let digit = increment_octal_digit(&0o12745usize, 3);
        assert_eq!(digit, 0o12045);
    }
}

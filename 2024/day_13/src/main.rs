fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let machines = load_data();
    let mut total = 0;
    for machine in machines {
        let tokens = compute_fewest_tokens_2(&machine);
        match tokens {
            Option::Some(tokens) => total += tokens,
            Option::None => (),
        }
    }
    total
}

fn solution_2() -> usize {
    let mut machines = load_data();
    for machine in machines.iter_mut() {
        machine.prize_x += 10000000000000;
        machine.prize_y += 10000000000000;
    }
    println!("Parsed {} machines.", machines.len());
    let mut total = 0;
    for (i, machine) in machines.iter().enumerate() {
        println!("Starting Machine {}", i);
        let tokens = compute_fewest_tokens_2(&machine);
        match tokens {
            Option::Some(tokens) => total += tokens,
            Option::None => (),
        }
    }
    total
}

fn compute_fewest_tokens_2(claw_machine: &ClawMachine) -> Option<usize> {
    // This really should have a detailed comment about the equation, but I'm not doing that.
    let part_1 = (claw_machine.prize_x * claw_machine.b_y) as i64;
    let part_2 = (claw_machine.b_x * claw_machine.prize_y) as i64;
    let numerator = part_1 - part_2;
    let part_1 = (claw_machine.a_x * claw_machine.b_y) as i64;
    let part_2 = (claw_machine.b_x * claw_machine.a_y) as i64;
    let denominator = part_1 - part_2;
    if numerator % denominator != 0 {
        return Option::None;
    }
    let a_button_count = numerator / denominator;
    let part_1 = claw_machine.a_y as i64 * a_button_count;
    let part_2 = claw_machine.prize_y as i64 - part_1;
    if part_2 % claw_machine.b_y as i64 != 0 {
        return Option::None;
    }
    let b_button_count = part_2 / claw_machine.b_y as i64;
    if a_button_count < 0 || b_button_count < 0 {
        return Option::None;
    }
    let a_button_count = a_button_count as usize;
    let b_button_count = b_button_count as usize;
    Option::Some((a_button_count * 3) + b_button_count)
}

fn compute_fewest_tokens(claw_machine: &ClawMachine) -> Option<usize> {
    let mut current_tokens = Option::None;
    let mut b_count = claw_machine.prize_x / claw_machine.b_x;
    let mut a_count = 0;
    while b_count > 0 {
        let a_move_x = a_count * claw_machine.a_x;
        let a_move_y = a_count * claw_machine.a_y;
        let b_move_x = b_count * claw_machine.b_x;
        let b_move_y = b_count * claw_machine.b_y;
        if a_move_x + b_move_x == claw_machine.prize_x && a_move_y + b_move_y == claw_machine.prize_y {
            match current_tokens {
                Option::None => current_tokens = Option::Some((a_count * 3) + b_count),
                Option::Some(current) => {
                    let next_tokens = (a_count * 3) + b_count;
                    if next_tokens < current {
                        current_tokens = Option::Some(next_tokens);
                    }
                }
            }
            b_count -= 1;
        } else if a_move_x + b_move_x > claw_machine.prize_x || a_move_y + b_move_y > claw_machine.prize_y {
            b_count -= 1;
        } else {
            a_count += 1;
        }
    }
    current_tokens
}

fn load_data() -> Vec<ClawMachine> {
    let mut machines = vec![];
    let mut lines = include_str!("../assets/input.txt")
        .lines();
    while let (Some(button_a), Some(button_b), Some(prize)) = (lines.next(), lines.next(), lines.next()) {
        let (a_x, a_y) = parse_button(button_a);
        let (b_x, b_y) = parse_button(button_b);
        let (prize_x, prize_y) = parse_prize(prize);
        machines.push(ClawMachine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });
        lines.next();
    }
    machines
}

fn parse_button(line: &str) -> (usize, usize) {
    let parts: Vec<&str> = line.split("X+").collect();
    let parts: Vec<&str> = parts[1].split(", Y+").collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn parse_prize(line: &str) -> (usize, usize) {
    let parts: Vec<&str> = line.split("X=").collect();
    let parts: Vec<&str> = parts[1].split(", Y=").collect();
    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

struct ClawMachine {
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize,
    prize_x: usize,
    prize_y: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_button_1() {
        let (x, y) = parse_button("Button A: X+14, Y+66");
        assert_eq!(x, 14);
        assert_eq!(y, 66);
    }

    #[test]
    fn parse_prize_1() {
        let (x, y) = parse_prize("Prize: X=9393, Y=7469");
        assert_eq!(x, 9393);
        assert_eq!(y, 7469);
    }
}

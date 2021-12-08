static DIRECTIONS: [&str; 4] = ["N", "E", "S", "W"];

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i32 {
    let commands = load_data();
    let moves = compute_moves(&commands);
    println!("{}, {}", moves.0, moves.1);
    moves.0.abs() + moves.1.abs()
}

fn solution_2() -> i32 {
    let commands = load_data();
    let moves = compute_moves_2(&commands);
    println!("{}, {}", moves.0, moves.1);
    moves.0.abs() + moves.1.abs()
}

fn compute_moves(commands: &Vec<(String, usize)>) -> (i32, i32) {
    let mut current_direction = "E";
    let mut direction_index: usize = 1;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    commands.iter()
        .map(|c| {
            match &c.0[..] {
                "N" => x += c.1 as i32,
                "S" => x -= c.1 as i32,
                "E" => y += c.1 as i32,
                "W" => y -= c.1 as i32,
                "R" => {
                    direction_index = (direction_index + (c.1/90)) % DIRECTIONS.len();
                    current_direction = DIRECTIONS[direction_index];
                },
                "L" => {
                    direction_index = (direction_index + ((360-c.1)/90)) % DIRECTIONS.len();
                    current_direction = DIRECTIONS[direction_index];
                },
                "F" => {
                    match current_direction {
                        "N" => x += c.1 as i32,
                        "S" => x -= c.1 as i32,
                        "E" => y += c.1 as i32,
                        "W" => y -= c.1 as i32,
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        })
        .count();
    (x, y)
}

fn compute_moves_2(commands: &Vec<(String, usize)>) -> (i32, i32) {
    let mut waypoint_x: i32 = 1;
    let mut waypoint_y: i32 = 10;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    commands.iter()
        .map(|c| {
            match &c.0[..] {
                "N" => waypoint_x += c.1 as i32,
                "S" => waypoint_x -= c.1 as i32,
                "E" => waypoint_y += c.1 as i32,
                "W" => waypoint_y -= c.1 as i32,
                "R" => {
                    for _ in 0..c.1/90 {
                        let temp = waypoint_y;
                        waypoint_y = waypoint_x;
                        waypoint_x = -temp;
                    }
                },
                "L" => {
                    for _ in 0..c.1/90 {
                        let temp = waypoint_y;
                        waypoint_y = -waypoint_x;
                        waypoint_x = temp;
                    }
                },
                "F" => {
                    x += waypoint_x * c.1 as i32;
                    y += waypoint_y * c.1 as i32;
                },
                _ => unreachable!(),
            }
        })
        .count();
    (x, y)
}

fn load_data() -> Vec<(String, usize)> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| {
            let direction = &s[0..1];
            let value = s[1..].parse::<usize>().unwrap_or(0);
            (direction.to_string(), value)
        })
        .collect()
}

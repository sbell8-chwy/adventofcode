use advent_common::load_file_as_lines;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i64 {
    let data = get_file_string();
    get_floor_number(data)
}

fn solution_2() -> i64 {
    let data = get_file_string();
    get_position_of_first_basement(data)
}

fn get_floor_number(data: String) -> i64 {
    let mut floor = 0;
    for c in data.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
    }
    floor
}

fn get_position_of_first_basement(data: String) -> i64 {
    let mut floor = 0;
    let mut position = 0;
    for c in data.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
        position += 1;
        if floor == -1 {
            break;
        }
    }
    position
}

fn get_file_string() -> String {
    let mut lines = load_file_as_lines("./assets/input.txt");
    lines.remove(0)
}

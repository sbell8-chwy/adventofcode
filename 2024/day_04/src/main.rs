fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let rows = data.len();
    let cols = data[0].len();
    let mut total_xmas = 0;
    for i in 0..rows {
        for j in 0..cols {
            let can_look_up = i > 2;
            let can_look_down = i < rows - 3;
            let can_look_left = j > 2;
            let can_look_right = j < cols - 3;
            if can_look_up && check_xmas(&data, [(i,j),(i-1,j),(i-2,j),(i-3,j)]) {
                total_xmas += 1;
            }
            if can_look_up && can_look_right && check_xmas(&data, [(i,j),(i-1,j+1),(i-2,j+2),(i-3,j+3)]) {
                total_xmas += 1;
            }
            if can_look_right && check_xmas(&data, [(i,j),(i,j+1),(i,j+2),(i,j+3)]) {
                total_xmas += 1;
            }
            if can_look_right && can_look_down && check_xmas(&data, [(i,j),(i+1,j+1),(i+2,j+2),(i+3,j+3)]) {
                total_xmas += 1;
            }
            if can_look_down && check_xmas(&data, [(i,j),(i+1,j),(i+2,j),(i+3,j)]) {
                total_xmas += 1;
            }
            if can_look_down && can_look_left && check_xmas(&data, [(i,j),(i+1,j-1),(i+2,j-2),(i+3,j-3)]) {
                total_xmas += 1;
            }
            if can_look_left && check_xmas(&data, [(i,j),(i,j-1),(i,j-2),(i,j-3)]) {
                total_xmas += 1;
            }
            if can_look_left && can_look_up && check_xmas(&data, [(i,j),(i-1,j-1),(i-2,j-2),(i-3,j-3)]) {
                total_xmas += 1;
            }
        }
    }
    total_xmas
}

fn solution_2() -> usize {
    let data = load_data();
    let rows = data.len();
    let cols = data[0].len();
    let mut total_xmas = 0;
    for i in 1..rows-1 {
        for j in 1..cols-1 {
            if data[i][j] == 'A' && check_x_mas(&data, (i,j)) {
                total_xmas += 1;
            }
        }
    }
    total_xmas
}

fn check_x_mas(data: &Vec<Vec<char>>, a_position: (usize, usize)) -> bool {
    let diag_1 = (data[a_position.0-1][a_position.1-1] == 'M' &&
        data[a_position.0+1][a_position.1+1] == 'S') ||
        (data[a_position.0-1][a_position.1-1] == 'S' &&
        data[a_position.0+1][a_position.1+1] == 'M');
    let diag_2 = (data[a_position.0-1][a_position.1+1] == 'M' &&
        data[a_position.0+1][a_position.1-1] == 'S') ||
        (data[a_position.0-1][a_position.1+1] == 'S' &&
        data[a_position.0+1][a_position.1-1] == 'M');
    diag_1 && diag_2
}

fn check_xmas(data: &Vec<Vec<char>>, positions: [(usize, usize); 4]) -> bool {
    data[positions[0].0][positions[0].1] == 'X' &&
        data[positions[1].0][positions[1].1] == 'M' &&
        data[positions[2].0][positions[2].1] == 'A' &&
        data[positions[3].0][positions[3].1] == 'S'
}

fn load_data() -> Vec<Vec<char>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

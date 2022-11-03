fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    0
}

fn solution_2() -> usize {
    0
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

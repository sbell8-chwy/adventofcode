fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let mut count = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            count = count + 1;
        }
    }
    count
}

fn solution_2() -> usize {
    let data = load_data();
    let mut count = 0;
    for i in 3..data.len() {
        if (data[i] + data[i - 1] + data[i - 2]) > (data[i - 1] + data[i - 2] + data[i - 3]) {
            count = count + 1;
        }
    }
    count
}

fn load_data() -> Vec<usize> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

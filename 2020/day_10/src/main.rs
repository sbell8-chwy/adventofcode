fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let mut data = load_data();
    data.sort();
    let mut one_count = 0;
    let mut three_count = 1;
    if data[0] == 1 {
        one_count += 1;
    }
    if data[0] == 3 {
        three_count += 1;
    }
    for i in 0..data.len()-1 {
        if data[i]+1 == data[i+1] {
            one_count += 1;
        }
        if data[i]+3 == data[i+1] {
            three_count += 1;
        }
    }
    one_count * three_count
}

fn solution_2() -> usize {
    let mut data = load_data();
    data.push(0);
    data.sort();
    let max: usize = *data.last().unwrap_or(&0);
    data.push(max + 3);
    let mut counts = vec![0; *data.last().unwrap_or(&0) + 1];
    counts[0] = 1;
    for i in data.iter().skip(1) {
        counts[*i] = match i {
            1 => counts[i-1],
            2 => counts[i-1] + counts[i-2],
            _ => counts[i-1] + counts[i-2] + counts[i-3],
        };
    }
    *counts.last().unwrap_or(&0usize)
}

fn load_data() -> Vec<usize> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.parse::<usize>().unwrap_or(0))
        .collect()
}

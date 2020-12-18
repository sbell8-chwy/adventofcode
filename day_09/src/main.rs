use std::iter::FromIterator;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    find_first_bad_number(&data, 25).0
}

fn solution_2() -> usize {
    let data = load_data();
    let bad_number = find_first_bad_number(&data, 25);
    for i in 0..bad_number.1-2 {
        for j in i + 1..bad_number.1-1 {
            let sub_data = &data[i..=j];
            if sub_data.iter().sum::<usize>() == bad_number.0 {
                return sub_data.iter().max().unwrap_or(&0usize) + sub_data.iter().min().unwrap_or(&0usize);

            }
        }
    }
    0
}

fn find_first_bad_number(numbers: &Vec<usize>, preamble_size: usize) -> (usize, usize) {
    let mut preamble = Vec::from_iter(numbers[0..preamble_size].iter().cloned());
    for i in preamble_size..numbers.len() {
        if !is_sum_contained(&preamble, numbers[i]) {
            return (numbers[i], i);
        }
        preamble.remove(0);
        preamble.push(numbers[i]);
    }
    (0, 0)
}

fn is_sum_contained(preamble: &Vec<usize>, number: usize) -> bool {
    for i in 0..preamble.len()-1 {
        for j in i+1..preamble.len() {
            if preamble[i] + preamble[j] == number {
                return true;
            }
        }
    }
    false
}

fn load_data() -> Vec<usize> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

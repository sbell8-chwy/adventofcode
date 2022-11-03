const BIT_WIDTH: usize = 12;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let bit_counts = count_bits(&data);
    let data_len = data.len();
    let mut gamma_rate = 0;
    for i in 0..bit_counts.len() {
        gamma_rate <<= 1;
        if bit_counts[i] >= data_len / 2 {
            gamma_rate = gamma_rate + 1;
        }
    }
    let epsilon_rate = 0x0FFF ^ gamma_rate;
    println!("{}", gamma_rate);
    println!("{}", epsilon_rate);

    gamma_rate * epsilon_rate
}

fn solution_2() -> usize {
    let data = load_data();
    0
}

fn count_bits(data: &Vec<String>) -> [usize; BIT_WIDTH] {
    let mut bit_counts: [usize; BIT_WIDTH] = [0; BIT_WIDTH];
    for i in 0..data.len() {
        for (j, c) in data[i].chars().enumerate() {
            match c {
                '1' => bit_counts[j] = bit_counts[j] + 1,
                '0' => (),
                _ => unreachable!(),
            }
        }
    }
    bit_counts
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

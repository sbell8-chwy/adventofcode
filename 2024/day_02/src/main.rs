#[derive(Debug, PartialEq, Eq)]
enum RowDirection {
    INCREASING,
    DECREASING,
    NONE
}
fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    data.iter()
        .map(|r| if is_safe(r.to_vec()) {1} else {0})
        .sum()
}

fn solution_2() -> usize {
    let data = load_data();
    data.iter()
        .map(|r| {
            if is_safe(r.to_vec()) {
                1
            } else {
                let mut found_safe = false;
                for i in 0..r.len() {
                    if is_safe([&r[..i], &r[i+1..]].concat()) {
                        found_safe = true;
                    }
                }
                if found_safe {1} else {0}
            }
        })
        .sum()
}

fn is_safe(data: Vec<usize>) -> bool {
    let mut safety = RowDirection::NONE;
    data.windows(2).all(|w| {
        match safety {
            RowDirection::NONE => {
                if w[0] == w[1] {
                    false
                } else if w[0] > w[1] && w[0] - w[1] < 4 {
                    safety = RowDirection::DECREASING;
                    true
                } else if w[1] > w[0] && w[1] - w[0] < 4 {
                    safety = RowDirection::INCREASING;
                    true
                } else {
                    false
                }
            }
            RowDirection::DECREASING => {
                w[0] > w[1] && w[0] - w[1] < 4
            }
            RowDirection::INCREASING => {
                w[1] > w[0] && w[1] - w[0] < 4
            }
        }
    })
}

fn load_data() -> Vec<Vec<usize>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| {
            let iter = s.split_whitespace();
            iter.map(|i| i.parse::<usize>().unwrap()).collect()
        })
        .collect()
}

use std::collections::{HashMap, VecDeque};

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u64 {
    let blink_count = 25;
    // let mut stone_vec = build_vec_of_queues(load_data(), blink_count);
    // blink_stones(&mut stone_vec, blink_count)
    blink_stones_smarter(&load_data(), blink_count)
}

fn solution_2() -> u64 {
    let blink_count = 75;
    // let mut stone_vec = build_vec_of_queues(load_data(), blink_count);
    // blink_stones(&mut stone_vec, blink_count)
    blink_stones_smarter(&load_data(), blink_count)
}

fn blink_stones_smarter(stones: &Vec<String>, blink_count: u64) -> u64 {
    let mut total = 0;
    let mut computed = HashMap::new();
    for stone in stones {
        total += recursive_blink(&stone, blink_count, &mut computed);
    }
    total
}

fn recursive_blink(stone: &String, depth: u64, computed: &mut HashMap<(String, u64), u64>) -> u64 {
    match computed.get(&(stone.to_owned(), depth)) {
        Some(value) => *value,
        None => {
            if depth == 0 {
                1
            } else {
                let new_stones = blink_stone(&stone);
                let mut total = 0;
                for new_stone in new_stones {
                    total += recursive_blink(&new_stone, depth - 1, computed);
                }
                computed.insert((stone.to_owned(), depth), total);
                total
            }
        }
    }
}

fn blink_stones(stone_vec: &mut Vec<VecDeque<String>>, blink_count: usize) -> u64 {
    let mut total = 0;
    loop {
        let mut found_stone = false;
        for i in (0..blink_count + 1).rev() {
            match stone_vec[i].pop_front() {
                Some(mut stone) => {
                    if i == blink_count {
                        total += 1;
                        found_stone = true;
                        // println!("Finish stone: {}", stone);
                        break;
                    } else {
                        // print!("Index: {}, starting stone {}, ", i, stone);
                        for new_stone in blink_stone(&mut stone) {
                            // print!("new stone {}, ", new_stone);
                            stone_vec[i+1].push_back(new_stone);
                        }
                        // println!("");
                        found_stone = true;
                        break;
                    }
                },
                None => (),
            }
        }
        if !found_stone {
            break;
        }
    }
    total
}

fn build_vec_of_queues(stones: Vec<String>, size: usize) -> Vec<VecDeque<String>> {
    let mut blink_vec = vec![];
    for _ in 0..size + 1 {
        blink_vec.push(VecDeque::new());
    }
    for stone in stones.into_iter() {
        blink_vec[0].push_back(stone);
    }
    blink_vec
}

fn blink_stone(stone: &String) -> Vec<String> {
    if stone == "0" {
        vec!["1".to_owned()]
    } else if stone.len() % 2 == 0 {
        let mut stone_1 = stone.clone();
        let mut stone_2 = stone_1.split_off(stone.len() / 2);
        stone_2 = stone_2.trim_start_matches('0').to_owned();
        if stone_2.len() == 0 {
            stone_2 = "0".to_owned();
        }
        vec![stone_1, stone_2]
    } else {
        let stone_value = stone.parse::<u64>().unwrap() * 2024;
        vec![stone_value.to_string()]
    }
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .trim_end()
        .split(" ")
        .map(|s| s.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stone_0_to_1() {
        let new_stone = blink_stone(&mut "0".to_owned());
        assert_eq!(new_stone.len(), 1);
        assert_eq!(new_stone[0], "1");
    }

    #[test]
    fn stone_split() {
        let new_stone = blink_stone(&mut "1234".to_owned());
        assert_eq!(new_stone.len(), 2);
        assert_eq!(new_stone[0], "12");
        assert_eq!(new_stone[1], "34");
    }

    #[test]
    fn stone_multiply_342() {
        let new_stone = blink_stone(&mut "342".to_owned());
        assert_eq!(new_stone.len(), 1);
        assert_eq!(new_stone[0], "692208");
    }
}

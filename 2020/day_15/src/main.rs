use std::collections::HashMap;

fn main() {
    println!("Solution 1 => {}", solution_1(2020));
    println!("Solution 2 => {}", solution_1(30_000_000));
}

fn solution_1(max_turns: usize) -> usize {
    let data = load_data();
    let mut spoken_map: HashMap<usize, usize> = HashMap::new();
    let mut last_spoken = data[data.len()-1];
    let mut turn = 0;
    for i in 0..data.len()-1 {
        turn += 1;
        spoken_map.insert(data[i], turn);
    }
    while turn < max_turns - 1 {
        turn += 1;
        match spoken_map.get(&last_spoken) {
            Some(v) => {
                let temp = last_spoken;
                last_spoken = turn - v;
                spoken_map.insert(temp, turn);
            },
            None => {
                spoken_map.insert(last_spoken, turn);
                last_spoken = 0;
            }
        }
        // println!("{}", last_spoken);
    }
    last_spoken
}

fn load_data() -> Vec<usize> {
    // vec![0, 3, 6]
    // vec![1, 3, 2]
    // vec![2, 1, 3]
    vec![5, 2, 8, 16, 18, 0, 1]
}

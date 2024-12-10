use std::collections::VecDeque;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let trail_map = load_data();
    let mut total = 0;
    for i in 0..trail_map.len() {
        for j in 0..trail_map[0].len() {
            if trail_map[i][j] == 0 {
                total += score_trailhead(&trail_map, (i, j), false);
            }
        }
    }
    total
}

fn solution_2() -> usize {
    let trail_map = load_data();
    let mut total = 0;
    for i in 0..trail_map.len() {
        for j in 0..trail_map[0].len() {
            if trail_map[i][j] == 0 {
                total += score_trailhead(&trail_map, (i, j), true);
            }
        }
    }
    total
}

fn score_trailhead(trail_map: &Vec<Vec<u32>>, trailhead: (usize, usize), count_unique: bool) -> usize {
    let mut total = 0;
    let mut deq = VecDeque::from(vec![trailhead]);
    while let Some(location) = deq.pop_front() {
        let location_value = trail_map[location.0][location.1];
        if  location_value == 9 {
            total += 1;
            continue;
        } else {
            for adjacent in get_adjacent_locations(location.0, location.1, trail_map.len()) {
                if trail_map[adjacent.0][adjacent.1] == location_value + 1 {
                    if count_unique || !deq.contains(&adjacent) {
                        deq.push_back(adjacent);
                    }
                }
            }
        }
    }
    total
}

fn get_adjacent_locations(x: usize, y: usize, size: usize) -> Vec<(usize, usize)> {
    let mut adjacent_locations = vec![];
    if x as i32 - 1 >= 0 {
        adjacent_locations.push((x-1, y));
    }
    if y + 1 < size {
        adjacent_locations.push((x, y+1));
    }
    if x + 1 < size {
        adjacent_locations.push((x+1, y));
    }
    if y as i32 - 1 >= 0 {
        adjacent_locations.push((x, y-1));
    }
    adjacent_locations
}

fn load_data() -> Vec<Vec<u32>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

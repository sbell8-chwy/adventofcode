use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let mut active_set = create_active_set(load_data());
    for _ in 0..6 {
        let neighbor_map = build_neighbor_map(&active_set);
        active_set = generate_new_active(&active_set, neighbor_map);
    }
    active_set.len()
}

fn solution_2() -> usize {
    0
}

fn create_active_set(data: Vec<String>) -> HashSet<(isize, isize, isize, isize)> {
    let mut active_set = HashSet::new();
    for (i, s) in data.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '#' {
                active_set.insert((i as isize, j as isize, 0_isize, 0_isize));
            }
        }
    }
    active_set
}

fn build_neighbor_map(
    active_set: &HashSet<(isize, isize, isize, isize)>,
) -> HashMap<(isize, isize, isize, isize), isize> {
    let mut neighbor_map = HashMap::new();
    for node in active_set.iter() {
        for x in node.0 - 1..=node.0 + 1 {
            for y in node.1 - 1..=node.1 + 1 {
                for z in node.2 - 1..=node.2 + 1 {
                    for w in node.3 - 1..=node.3 + 1 {
                        if node.0 == x && node.1 == y && node.2 == z && node.3 == w {
                            continue;
                        }
                        *neighbor_map.entry((x, y, z, w)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    neighbor_map
}

fn generate_new_active(
    active_set: &HashSet<(isize, isize, isize, isize)>,
    neighbor_map: HashMap<(isize, isize, isize, isize), isize>,
) -> HashSet<(isize, isize, isize, isize)> {
    let mut new_active = HashSet::new();
    for (k, v) in neighbor_map.iter() {
        match v {
            3 => drop(new_active.insert((k.0, k.1, k.2, k.3))),
            2 => {
                if active_set.contains(&k) {
                    new_active.insert((k.0, k.1, k.2, k.3));
                }
            }
            _ => (),
        }
    }
    new_active
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

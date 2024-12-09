use std::collections::HashSet;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    count_antinodes(&load_data())
}

fn solution_2() -> usize {
    let map = load_data();
    let node_pairs = find_node_pairs(&map);
    compute_all_antinodes(&node_pairs, map.len())
}

fn find_node_pairs(map: &Vec<Vec<char>>) -> Vec<((usize, usize), (usize, usize))> {
    let mut nodes = vec![];
    let length = map.len();
    for i in 0..length {
        for j in 0..length {
            if map[i][j] != '.' {
                let current_node = map[i][j];
                for k in i..length {
                    for l in 0..length {
                        if k == i && l <= j {
                            continue;
                        }
                        if map[k][l] == current_node {
                            nodes.push(((i,j),(k,l)));
                        }
                    }
                }
            }
        }
    }
    nodes
}

fn compute_all_antinodes(node_pairs: &Vec<((usize, usize), (usize, usize))>, length: usize) -> usize {
    let mut antinodes = HashSet::new();
    for node_pair in node_pairs.iter() {
        antinodes.insert(node_pair.0);
        antinodes.insert(node_pair.1);
        for node in compute_multi_antinodes(node_pair.0.0, node_pair.0.1, node_pair.1.0, node_pair.1.1, length) {
            antinodes.insert(node);
        }
    }
    antinodes.len()
}

fn count_antinodes(map: &Vec<Vec<char>>) -> usize {
    let mut antinodes = HashSet::new();
    let length = map.len();
    for i in 0..length {
        for j in 0..length {
            if map[i][j] != '.' {
                let current_node = map[i][j];
                for k in i..length {
                    for l in 0..length {
                        if k == i && l <= j {
                            continue;
                        }
                        if map[k][l] == current_node {
                            for an in compute_antinode(i, j, k, l, length) {
                                antinodes.insert(an);
                            }
                        }
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn compute_multi_antinodes(i: usize, j: usize, k: usize, l:usize, length: usize) -> Vec<(usize, usize)> {
    let mut antinodes = vec![];
    let i: i64 = i as i64;
    let j: i64 = j as i64;
    let k: i64 = k as i64;
    let l: i64 = l as i64;
    let length: i64 = length as i64;
    let dx = i - k;
    let dy = j - l;

    let mut x = i + dx;
    let mut y = j + dy;
    while x >= 0 && x < length && y >=0 && y < length {
        antinodes.push((x as usize, y as usize));
        x += dx;
        y += dy;
    }
    let mut x = k - dx;
    let mut y = l - dy;
    while x >= 0 && x < length && y >=0 && y < length {
        antinodes.push((x as usize, y as usize));
        x -= dx;
        y -= dy;
    }
    antinodes
}

fn compute_antinode(i: usize, j: usize, k: usize, l:usize, length: usize) -> Vec<(usize, usize)> {
    let mut antinodes = vec![];
    let i: i64 = i as i64;
    let j: i64 = j as i64;
    let k: i64 = k as i64;
    let l: i64 = l as i64;
    let length: i64 = length as i64;
    let x1 = i + (i - k);
    let y1 = j + (j - l);
    if x1 >= 0 && x1 < length && y1 >= 0 && y1 < length {
        antinodes.push((x1 as usize, y1 as usize));
    }
    let x2 = k + (k - i);
    let y2 = l + (l - j);
    if x2 < length && x2 >= 0 && y2 < length && y2 >= 0 {
        antinodes.push((x2 as usize, y2 as usize));
    }
    antinodes
}

fn load_data() -> Vec<Vec<char>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

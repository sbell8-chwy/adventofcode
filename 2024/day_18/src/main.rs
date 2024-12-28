use std::collections::VecDeque;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {:?}", solution_2());
}

fn solution_1() -> usize {
    let falling_memory = load_data();
    let map = map_corrupt_space(71, 71, &falling_memory, 1024);
    print_map(&map);
    let path = find_shortest_path(&map, 70);
    path.len()
}

fn solution_2() -> (usize, usize) {
    let falling_memory = load_data();
    let mut map = map_corrupt_space(71, 71, &falling_memory, 1024);
    for i in 1024..falling_memory.len() {
        println!("{}", i);
        push_falling_byte(&mut map, falling_memory[i]);
        let path = find_shortest_path(&map, 70);
        if path.len() == 0 {
            return falling_memory[i];
        }
    }
    (0, 0)
}

fn find_shortest_path(map: &Vec<Vec<char>>, map_size: usize) -> Vec<(usize, usize)> {
    let mut seen = vec![];
    let mut paths: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
    paths.push_back(vec![(0, 0)]);
    seen.push((0,0));
    while let Some(path) = paths.pop_front() {
        let point = path.last().unwrap();
        if point == &(map_size, map_size) {
            return path;
        }
        if point.0 > 0 && !seen.contains(&(point.0-1, point.1)) && map[point.1][point.0-1] == '.' {
            seen.push((point.0-1, point.1));
            paths.push_back(new_path(&path, (point.0-1,point.1)));
        }
        if point.1 > 0 && !seen.contains(&(point.0, point.1-1)) && map[point.1-1][point.0] == '.' {
            seen.push((point.0, point.1-1));
            paths.push_back(new_path(&path, (point.0,point.1-1)));
        }
        if point.0 < map_size && !seen.contains(&(point.0+1, point.1)) && map[point.1][point.0+1] == '.' {
            seen.push((point.0+1, point.1));
            paths.push_back(new_path(&path, (point.0+1,point.1)));
        }
        if point.1 < map_size && !seen.contains(&(point.0, point.1+1)) && map[point.1+1][point.0] == '.' {
            seen.push((point.0, point.1+1));
            paths.push_back(new_path(&path, (point.0,point.1+1)));
        }
    }
    vec![]
}

fn new_path(path: &Vec<(usize, usize)>, point: (usize, usize)) -> Vec<(usize, usize)> {
    let mut new_path = path.clone();
    new_path.push(point);
    new_path
}

fn map_corrupt_space(x: usize, y: usize, data: &Vec<(usize, usize)>, count: usize) -> Vec<Vec<char>> {
    let mut map = build_empty_map(x, y);
    for i in 0..count {
        push_falling_byte(&mut map, data[i]);
    }
    map
}

fn push_falling_byte(map: &mut Vec<Vec<char>>, data: (usize, usize)) {
    map[data.1][data.0] = '#';
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn build_empty_map(x: usize, y: usize) -> Vec<Vec<char>> {
    let mut map = vec![];
    for i in 0..x {
        map.push(vec![]);
        for _ in 0..y {
            map[i].push('.');
        }
    }
    map
}

fn load_data() -> Vec<(usize, usize)> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|l| {
            let parts: Vec<usize> = l.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect()
}

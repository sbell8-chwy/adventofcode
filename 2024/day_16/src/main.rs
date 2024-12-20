use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
};

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let map = load_data();
    compute_path(&map).1
}

fn solution_2() -> usize {
    let map = load_data();
    let (cost, count) = find_depth_first(&map);
    println!("Cost: {}", cost);
    count
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        println!("{}", line.into_iter().collect::<String>());
    }
}
// fn print_map(map: &Vec<Vec<Option<usize>>>) {
    // for line in map {
        // for x in line {
            // match x {
                // Some(val) => print!("{} ", format!("{:0>5}", val)),
                // None => print!("xxxxx "),
            // }
        // }
        // println!();
    // }
// }

fn find_depth_first(map: &Vec<Vec<char>>) -> (usize, usize) {
    let start = find_in_map(&map, 'S');
    let end = find_in_map(&map, 'E');
    let mut seen = HashMap::new();
    let start_position = Position::new(start, Direction::EAST);
    seen.insert(start_position, 0);
    let found_paths = search_next_node(&map, PositionWithCost::new(start_position, 0), &end, &mut seen);
    let mut touched_locations = HashSet::new();
    for path in found_paths.iter() {
        for loc in path {
            touched_locations.insert(loc.position.location);
        }
    }
    (found_paths[0].back().unwrap().cost, touched_locations.len())
}

fn search_next_node(map: &Vec<Vec<char>>, current: PositionWithCost, end: &(usize, usize), seen: &mut HashMap<Position, usize>) -> Vec<VecDeque<PositionWithCost>> {
    if &current.position.location == end {
        seen.insert(current.position, current.cost);
        println!("Found End {:?}", current);
        return vec![VecDeque::from([current])];
    }
    let mut paths = vec![];
    let possible = [current.turn_left(), current.turn_right(), current.move_forward()];
    let mut smallest_value = 0;
    for next in possible {
        if !seen.contains_key(&next.position) || seen.get(&next.position).unwrap() >= &next.cost {
            if map[next.position.location.0][next.position.location.1] != '#' && next.cost < 95438 {
                seen.insert(next.position, next.cost);
                let found = search_next_node(map, next, end, seen);
                for f in found.iter() {
                    if smallest_value == 0 || f.back().unwrap().cost < smallest_value {
                        smallest_value = f.back().unwrap().cost;
                    }
                }
                for mut f in found {
                    if f.back().unwrap().cost == smallest_value {
                        f.push_front(current);
                        paths.push(f);
                    }
                }
            }
        }
    }
    paths
}

fn compute_path(map: &Vec<Vec<char>>) -> (Vec<Vec<Option<usize>>>, usize) {
    let start = find_in_map(&map, 'S');
    let end = find_in_map(&map, 'E');
    let start_position = Position::new(start, Direction::EAST);
    let mut open_set = BTreeSet::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let mut cost_map = build_cost_map(&map);
    let mut final_cost = 0;
    cost_map[start.0][start.1] = Some(0);
    open_set.insert(PositionWithCost::new(start_position, 0));
    g_score.insert(start_position, 0);
    while open_set.len() > 0 {
        let current = open_set.pop_first().unwrap();
        if current.position.location == end {
            if final_cost == 0 {
                final_cost = current.cost;
                cost_map[current.position.location.0][current.position.location.1] = Some(current.cost);
            }
        }
        for neighbor in [current.turn_left(), current.turn_right()] {
            if !g_score.contains_key(&neighbor.position)
                || g_score[&neighbor.position] > neighbor.cost
            {
                g_score.insert(neighbor.position, neighbor.cost);
                f_score.insert(
                    neighbor.position,
                    neighbor.cost + estimate_cost(neighbor.position.location, end),
                );
                open_set.insert(neighbor);
            }
        }
        let neighbor = current.move_forward();
        if map[neighbor.position.location.0][neighbor.position.location.1] != '#' {
            let loc = current.position.location;
            match cost_map[loc.0][loc.1] {
                Some(x) => {
                    if x > current.cost {
                        cost_map[loc.0][loc.1] = Some(current.cost);
                    }
                }
                None => cost_map[loc.0][loc.1] = Some(current.cost),
            }
            if !g_score.contains_key(&neighbor.position)
                || g_score[&neighbor.position] > neighbor.cost
            {
                g_score.insert(neighbor.position, neighbor.cost);
                f_score.insert(
                    neighbor.position,
                    neighbor.cost + estimate_cost(neighbor.position.location, end),
                );
                open_set.insert(neighbor);
            }
        }
    }
    (cost_map, final_cost)
}

fn build_cost_map(map: &Vec<Vec<char>>) -> Vec<Vec<Option<usize>>> {
    let mut cost_map = vec![];
    for v in map {
        cost_map.push(vec![Option::None; v.len()]);
    }
    cost_map
}

fn estimate_cost(from: (usize, usize), to: (usize, usize)) -> usize {
    if from.0 == to.0 {
        if from.1 > to.1 {
            return from.1 - to.1;
        } else {
            return to.1 - from.1;
        }
    } else if from.1 == to.1 {
        if from.0 > to.0 {
            return from.0 - to.0;
        } else {
            return to.0 - from.0;
        }
    } else {
        let x = if from.0 > to.0 {
            from.0 - to.0
        } else {
            to.0 - from.0
        };
        let y = if from.1 > to.1 {
            from.1 - to.1
        } else {
            to.1 - from.1
        };
        return x + y + 1000;
    }
}

fn find_in_map(map: &Vec<Vec<char>>, item: char) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == item {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn load_data() -> Vec<Vec<char>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

#[derive(Debug, Clone, Eq, Copy, PartialEq, PartialOrd, Hash)]
struct Position {
    location: (usize, usize),
    facing: Direction,
}

#[derive(Debug, Clone, Eq, Copy, PartialEq, PartialOrd, Hash)]
struct PositionWithCost {
    position: Position,
    cost: usize,
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Ord for PositionWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost, self.position.location, self.position.facing).cmp(&(
            other.cost,
            other.position.location,
            other.position.facing,
        ))
    }
}

impl PositionWithCost {
    fn new(position: Position, cost: usize) -> Self {
        PositionWithCost { position, cost }
    }
    fn move_forward(&self) -> PositionWithCost {
        PositionWithCost {
            position: self.position.move_forward(),
            cost: self.cost + 1,
        }
    }

    fn turn_left(&self) -> PositionWithCost {
        PositionWithCost {
            position: self.position.turn_left(),
            cost: self.cost + 1000,
        }
    }

    fn turn_right(&self) -> PositionWithCost {
        PositionWithCost {
            position: self.position.turn_right(),
            cost: self.cost + 1000,
        }
    }
}

impl Position {
    fn new(location: (usize, usize), facing: Direction) -> Self {
        Self { location, facing }
    }
    fn move_forward(&self) -> Position {
        Position {
            location: match self.facing {
                Direction::NORTH => (self.location.0 - 1, self.location.1),
                Direction::WEST => (self.location.0, self.location.1 - 1),
                Direction::SOUTH => (self.location.0 + 1, self.location.1),
                Direction::EAST => (self.location.0, self.location.1 + 1),
            },
            facing: self.facing.clone(),
        }
    }

    fn turn_left(&self) -> Position {
        Position {
            location: self.location,
            facing: match self.facing {
                Direction::NORTH => Direction::WEST,
                Direction::WEST => Direction::SOUTH,
                Direction::SOUTH => Direction::EAST,
                Direction::EAST => Direction::NORTH,
            },
        }
    }

    fn turn_right(&self) -> Position {
        Position {
            location: self.location,
            facing: match self.facing {
                Direction::NORTH => Direction::EAST,
                Direction::WEST => Direction::NORTH,
                Direction::SOUTH => Direction::WEST,
                Direction::EAST => Direction::SOUTH,
            },
        }
    }
}

use std::collections::HashSet;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let (mut map, moves) = load_data();
    simulate_robot(&mut map, &moves);
    compute_cargo_gps(&map)
}

fn solution_2() -> usize {
    let (mut map, moves) = load_data_2();
    simulate_robot_2(&mut map, &moves);
    compute_cargo_gps(&map)
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn compute_cargo_gps(map: &Vec<Vec<char>>) -> usize {
    print_map(map);
    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' || map[i][j] == '[' {
                total += (i*100) + j;
                // println!("Computing {}, {} = {}, new total: {}", i, j, (i*100)+j, total);
            }
        }
    }
    total
}

fn simulate_robot_2(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    let mut robot_x = 0;
    let mut robot_y = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                robot_x = i;
                robot_y = j;
            }
        }
    }

    for c in moves {
        // print_map(&map);
        // println!("{}", c);
        match c {
            '<' => {
                let mut moves = get_movable_left(&map, (robot_x, robot_y));
                if moves.len() != 0 {
                    while let Some(m) = moves.pop() {
                        map[m.0][m.1-1] = map[m.0][m.1];
                        map[m.0][m.1] = '.';
                    }
                    robot_y -= 1;
                }
            },
            '>' => {
                let mut moves = get_movable_right(&map, (robot_x, robot_y));
                if moves.len() != 0 {
                    while let Some(m) = moves.pop() {
                        map[m.0][m.1+1] = map[m.0][m.1];
                        map[m.0][m.1] = '.';
                    }
                    robot_y += 1;
                }
            },
            '^' => {
                let mut moves = get_movable_up(&map, (robot_x, robot_y));
                if moves.len() != 0 {
                    // let print = moves.len() > 7;
                    // if print {
                        // print_map(&map);
                        // println!("{}", c);
                    // }
                    while let Some(m) = moves.pop() {
                        map[m.0-1][m.1] = map[m.0][m.1];
                        map[m.0][m.1] = '.';
                    }
                    robot_x -= 1;
                    // if print {
                        // print_map(&map);
                    // }
                }
            },
            'v' => {
                let mut moves = get_movable_down(&map, (robot_x, robot_y));
                if moves.len() != 0 {
                    // let print = moves.len() > 3;
                    // if print {
                        // print_map(&map);
                        // println!("{}", c);
                    // }
                    while let Some(m) = moves.pop() {
                        map[m.0+1][m.1] = map[m.0][m.1];
                        map[m.0][m.1] = '.';
                    }
                    robot_x += 1;
                    // if print {
                        // print_map(&map);
                    // }
                }
            },
            _ => ()
        }
    }
}

fn get_movable_up(map: &Vec<Vec<char>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    let mut current_movable = HashSet::new();
    current_movable.insert((location.0, location.1));
    while current_movable.len() > 0 {
        let mut next_movable = HashSet::new();
        for m in current_movable.iter() {
            if map[m.0-1][m.1] == '#' {
                return vec![];
            } else if map[m.0-1][m.1] == '[' {
                next_movable.insert((m.0-1, m.1));
                next_movable.insert((m.0-1, m.1+1));
            } else if map[m.0-1][m.1] == ']' {
                next_movable.insert((m.0-1, m.1));
                next_movable.insert((m.0-1, m.1-1));
            }
        }
        movable.extend(current_movable.into_iter());
        current_movable = next_movable;
    }
    movable
}

fn get_movable_down(map: &Vec<Vec<char>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    let mut current_movable = HashSet::new();
    current_movable.insert((location.0, location.1));
    while current_movable.len() > 0 {
        let mut next_movable = HashSet::new();
        for m in current_movable.iter() {
            if map[m.0+1][m.1] == '#' {
                return vec![];
            } else if map[m.0+1][m.1] == '[' {
                next_movable.insert((m.0+1, m.1));
                next_movable.insert((m.0+1, m.1+1));
            } else if map[m.0+1][m.1] == ']' {
                next_movable.insert((m.0+1, m.1));
                next_movable.insert((m.0+1, m.1-1));
            }
        }
        movable.extend(current_movable.into_iter());
        current_movable = next_movable;
    }
    movable
}

fn get_movable_left(map: &Vec<Vec<char>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    movable.push((location.0, location.1));
    for i in (0..location.1).rev() {
        if map[location.0][i] == '#' {
            return vec![];
        } else if map[location.0][i] == '[' || map[location.0][i] == ']' {
            movable.push((location.0, i));
        } else if map[location.0][i] == '.' {
            break;
        }
    }
    movable
}

fn get_movable_right(map: &Vec<Vec<char>>, location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut movable = vec![];
    movable.push((location.0, location.1));
    for i in (location.1+1)..map[0].len() {
        if map[location.0][i] == '#' {
            return vec![];
        } else if map[location.0][i] == '[' || map[location.0][i] == ']' {
            movable.push((location.0, i));
        } else if map[location.0][i] == '.' {
            break;
        }
    }
    movable
}

fn simulate_robot(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    let mut robot_x = 0;
    let mut robot_y = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                robot_x = i;
                robot_y = j;
            }
        }
    }

    for c in moves {
        // for line in map.iter() {
            // println!("{}", line.into_iter().collect::<String>());
        // }
        // println!("{}", c);
        match c {
            '<' => {
                for i in (0..robot_y).rev() {
                    if map[robot_x][i] == '.' {
                        for j in i..robot_y {
                            map[robot_x][j] = map[robot_x][j+1];
                            map[robot_x][j+1] = '.';
                        }
                        robot_y -= 1;
                        break;
                    } else if map[robot_x][i] == '#' {
                        break;
                    }
                }
            },
            '>' => {
                for i in (robot_y+1)..map[0].len() {
                    if map[robot_x][i] == '.' {
                        for j in (robot_y..i).rev() {
                            map[robot_x][j+1] = map[robot_x][j];
                            map[robot_x][j] = '.';
                        }
                        robot_y += 1;
                        break;
                    } else if map[robot_x][i] == '#' {
                        break;
                    }
                }
            },
            '^' => {
                for i in (0..robot_x).rev() {
                    if map[i][robot_y] == '.' {
                        for j in i..robot_x {
                            map[j][robot_y] = map[j+1][robot_y];
                            map[j+1][robot_y] = '.';
                        }
                        robot_x -= 1;
                        break;
                    } else if map[i][robot_y] == '#' {
                        break;
                    }
                }
            },
            'v' => {
                for i in (robot_x+1)..map.len() {
                    if map[i][robot_y] == '.' {
                        for j in (robot_x..i).rev() {
                            map[j+1][robot_y] = map[j][robot_y];
                            map[j][robot_y] = '.';
                        }
                        robot_x += 1;
                        break;
                    } else if map[i][robot_y] == '#' {
                        break;
                    }
                }
            },
            _ => ()
        }
    }
}

fn load_data_2() -> (Vec<Vec<char>>, Vec<char>) {
    let mut map = vec![];
    let mut moves = vec![];
    let mut reading_moves = false;

    let mut lines = include_str!("../assets/input.txt")
        .lines();
    while let Some(line) = lines.next() {
        if reading_moves {
            for c in line.trim().chars().into_iter() {
                moves.push(c);
            }
        } else {
            let char_vec: Vec<char> = line
                .chars()
                .flat_map(|c| {
                    match c {
                        '#' => "##".to_owned().chars().collect::<Vec<char>>(),
                        'O' => "[]".to_owned().chars().collect::<Vec<char>>(),
                        '.' => "..".to_owned().chars().collect::<Vec<char>>(),
                        '@' => "@.".to_owned().chars().collect::<Vec<char>>(),
                        _ => "".to_owned().chars().collect(),
                    }
                })
                .collect();
            if line.len() == 0 {
                reading_moves = true;
            } else {
                map.push(char_vec);
            }
        }
    }
    (map, moves)
}

fn load_data() -> (Vec<Vec<char>>, Vec<char>) {
    let mut map = vec![];
    let mut moves = vec![];
    let mut reading_moves = false;

    let mut lines = include_str!("../assets/tiny_input.txt")
        .lines();
    while let Some(line) = lines.next() {
        if reading_moves {
            for c in line.trim().chars().into_iter() {
                moves.push(c);
            }
        } else {
            let char_vec: Vec<char> = line.chars().collect();
            if line.len() == 0 {
                reading_moves = true;
            } else {
                map.push(char_vec);
            }
        }
    }
    (map, moves)
}

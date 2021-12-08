fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let mut seats = load_data();
    while run_cycle(&mut seats, false) {
        // println!("-------------------------------------------------------------------");
        // seats.iter()
            // .map(|l| println!("{:?}", l))
            // .count();
        // println!("-------------------------------------------------------------------");
    }
    count_seats(&seats)
}

fn solution_2() -> usize {
    let mut seats = load_data();
    while run_cycle(&mut seats, true) {
        // println!("-------------------------------------------------------------------");
        // seats.iter()
            // .map(|l| println!("{:?}", l))
            // .count();
        // println!("-------------------------------------------------------------------");
    }
    count_seats(&seats)
}

fn count_seats(seats: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            if seats[i][j] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn run_cycle(seats: &mut Vec<Vec<char>>, part_two: bool) -> bool {
    let mut changes: Vec<(char, (usize, usize))> = vec![];
    let seat_limit = if part_two {
        5
    } else {
        4
    };
    for i in 0..seats.len() {
        for j in 0..seats[i].len() {
            let occupied = if part_two {
                count_occupied_seats_2(seats, i, j)
            } else {
                count_occupied_seats(seats, i, j)
            };
            match seats[i][j] {
                'L' => {
                    if occupied == 0 {
                        changes.push(('#', (i, j)));
                    }
                },
                '#' => {
                    if occupied >= seat_limit {
                        changes.push(('L', (i, j)));
                    }
                },
                '.' => (),
                _ => unreachable!(),
            }
        }
    }
    changes.iter()
        .map(|c| seats[c.1.0][c.1.1] = c.0)
        .count();
    changes.len() > 0
}

fn count_occupied_seats(seats: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut occupied = 0;
    for i in x.saturating_sub(1)..=x+1 {
        for j in y.saturating_sub(1)..=y+1 {
            if i == x && j == y {
                continue;
            }
            if i < seats.len() {
                if j < seats[i].len() {
                    if seats[i][j] == '#' {
                        occupied += 1;
                    }
                }
            }
        }
    }
    occupied
}

fn count_occupied_seats_2(seats: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut occupied = 0;
    let print = false;// x == 7 && y == 9;
    if print { println!("checking 7, 9"); }
    for i in (0..y).rev() {
        if seats[x][i] == '#' {
            if print { println!("1 found occupied {}, {}", x, i); }
            occupied += 1;
            break;
        }
        if seats[x][i] == 'L' {
            break;
        }
    }
    for i in y+1..seats[0].len(){
        if seats[x][i] == '#' {
            if print { println!("2 found occupied {}, {}", x, i); }
            occupied += 1;
            break;
        }
        if seats[x][i] == 'L' {
            break;
        }
    }
    for i in (0..x).rev() {
        if seats[i][y] == '#' {
            if print { println!("3 found occupied {}, {}", i, y); }
            occupied += 1;
            break;
        }
        if seats[i][y] == 'L' {
            break;
        }
    }
    for i in x+1..seats.len(){
        if seats[i][y] == '#' {
            if print { println!("4 found occupied {}, {}", i, y); }
            occupied += 1;
            break;
        }
        if seats[i][y] == 'L' {
            break;
        }
    }
    let mut count = 1;
    loop {
        let i = x.checked_sub(count);
        let j = y.checked_sub(count);
        match i {
            Some(ival) => {
                match j {
                    Some(jval) => {
                        if seats[ival][jval] == '#' {
                            if print { println!("5 found occupied {}, {}", ival, jval); }
                            occupied += 1;
                            break;
                        }
                        if seats[ival][jval] == 'L' {
                            break;
                        }
                    },
                    None => break,
                }
            },
            None => break,
        }
        count += 1;
    }
    count = 1;
    loop {
        let i = x + count;
        let j = y + count;
        if i < seats.len() {
            if j < seats[i].len() {
                if seats[i][j] == '#' {
                    if print { println!("6 found occupied {}, {}", i, j); }
                    occupied += 1;
                    break;
                }
                if seats[i][j] == 'L' {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
        count += 1;
    }
    count = 1;
    loop {
        let i = x.checked_sub(count);
        let j = y + count;
        match i {
            Some(ival) => {
                if j < seats[ival].len() {
                    if seats[ival][j] == '#' {
                        if print { println!("7 found occupied {}, {}", ival, j); }
                        occupied += 1;
                        break;
                    }
                    if seats[ival][j] == 'L' {
                        break;
                    }
                } else {
                    break;
                }
            },
            None => break,
        }
        count += 1;
    }
    count = 1;
    loop {
        let i = x + count;
        let j = y.checked_sub(count);
        if i < seats.len() {
            match j {
                Some(jval) => {
                    if seats[i][jval] == '#' {
                        if print { println!("8 found occupied {}, {}, {}", i, jval, count); }
                        occupied += 1;
                        break;
                    }
                    if seats[i][jval] == 'L' {
                        break;
                    }
                },
                None => break,
            }
        } else {
            break;
        }
        count += 1;
    }
    occupied
}

fn load_data() -> Vec<Vec<char>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u32 {
    let mut room = load_data();
    let guard_location = find_guard_location(&room);
    walk_guard(&mut room, guard_location);
    let count = count_x(room);
    count
}

fn solution_2() -> usize {
    let clean_room = load_data();
    let guard_location = find_guard_location(&clean_room);
    let mut count = 0;
    for i in 0..clean_room.len() {
        for j in 0..clean_room[0].len() {
            let mut room = clean_room.to_vec(); // load_data();
            room[i][j] = '#';
            println!("Testing obstical in locaiton {}, {}", i, j);
            if !walk_guard(&mut room, guard_location) {
                count += 1;
            }
        }
    }
    count
}

fn walk_guard(room: &mut Vec<Vec<char>>, mut guard_position: (u32,u32)) -> bool {
    let directions = [(-1i8,0i8),(0i8,1i8),(1i8,0i8),(0i8,-1i8)];
    let mut current_direction = 0;
    room[guard_position.0 as usize][guard_position.1 as usize] = 'X';
    let mut seen_spots: Vec<(u32, u32, i8, i8)> = vec![];
    seen_spots.push((guard_position.0, guard_position.1, directions[0].0, directions[0].1));
    while (guard_position.0 as usize) > 0 &&
    (guard_position.0 as usize) < room.len() - 1 &&
    (guard_position.1 as usize) > 0 &&
    (guard_position.1 as usize) < room[0].len() - 1 {
        let new_x = get_index(guard_position.0, directions[current_direction].0);
        let new_y = get_index(guard_position.1, directions[current_direction].1);
        if room[new_x][new_y] == '#' {
            current_direction = (current_direction + 1) % directions.len();
            if seen_spots.contains(&(guard_position.0, guard_position.1, directions[current_direction].0, directions[current_direction].1)) {
                return false;
            } else {
                seen_spots.push((guard_position.0, guard_position.1, directions[current_direction].0, directions[current_direction].1));
            }
        } else {
            guard_position.0 = ((guard_position.0 as i32) + (directions[current_direction].0 as i32)) as u32;
            guard_position.1 = ((guard_position.1 as i32) + (directions[current_direction].1 as i32)) as u32;
            room[guard_position.0 as usize][guard_position.1 as usize] = 'X';
            if seen_spots.contains(&(guard_position.0, guard_position.1, directions[current_direction].0, directions[current_direction].1)) {
                return false;
            } else {
                seen_spots.push((guard_position.0, guard_position.1, directions[current_direction].0, directions[current_direction].1));
            }
        }
    }
    true
}

fn count_x(room: Vec<Vec<char>>) -> u32 {
    let mut total = 0;
    for row in room.iter() {
        for c in row.iter() {
            if c == &'X' {
                total += 1;
            }
        }
    }
    total
}

fn get_index(index: u32, adjust: i8) -> usize {
    let new_index = (index as i32) + (adjust as i32);
    new_index as usize
}

fn find_guard_location(room: &Vec<Vec<char>>) -> (u32, u32) {
    for i in 0..room.len() {
        for j in 0..room[0].len() {
            if room[i][j] == '^' {
                return (i as u32,j as u32);
            }
        }
    }
    panic!("Failed to find guard");
}

fn load_data() -> Vec<Vec<char>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}

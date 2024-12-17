fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i64 {
    let map_size = (101, 103);
    let robots = load_data();
    let locations = compute_robot_end(&robots, map_size, 100);
    let counts = count_locations_in_quadrants(&locations, map_size);
    counts.0 * counts.1 * counts.2 * counts.3
}

fn solution_2() -> i64 {
    let map_size = (101, 103);
    let mut robots = load_data();
    for i in 1..10_000 {
        move_robots(&mut robots, map_size);
        print_robots(&robots, map_size, i);
    }
    0
}

fn move_robots(robots: &mut Vec<Robot>, map_size: (i64, i64)) {
    for robot in robots.iter_mut() {
        let new_x = (robot.start.0 + robot.velocity.0).rem_euclid(map_size.0);
        let new_y = (robot.start.1 + robot.velocity.1).rem_euclid(map_size.1);
        robot.start.0 = new_x;
        robot.start.1 = new_y;
    }
}

fn print_robots(robots: &Vec<Robot>, map_size: (i64, i64), tick_count: i64) {
    let mut board = vec![];
    for i in 0..map_size.0 {
        board.push(vec![]);
        for _ in 0..map_size.1 {
            board[i as usize].push('.');
        }
    }
    for robot in robots {
        board[robot.start.0 as usize][robot.start.1 as usize] = '#';
    }
    println!("Board at tick count {}", tick_count);
    for line in board {
        println!("{}", line.iter().cloned().collect::<String>());
    }
}

fn count_locations_in_quadrants(locations: &Vec<(i64, i64)>, map_size: (i64, i64)) -> (i64, i64, i64, i64) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for location in locations {
        if location.0 == map_size.0 / 2 || location.1 == map_size.1 / 2 {
            continue;
        }
        if location.0 < map_size.0 / 2 {
            if location.1 < map_size.1 / 2 {
                q1 += 1;
            } else {
                q3 += 1;
            }
        } else {
            if location.1 < map_size.1 / 2 {
                q2 += 1;
            } else {
                q4 += 1;
            }
        }
    }

    (q1, q2, q3, q4)
}

fn compute_robot_end(robots: &Vec<Robot>, map_size: (i64, i64), ticks: i64) -> Vec<(i64, i64)> {
    let mut end_locations = vec![];
    for robot in robots {
        let new_x = (robot.start.0 + (robot.velocity.0 * ticks)).rem_euclid(map_size.0);
        let new_y = (robot.start.1 + (robot.velocity.1 * ticks)).rem_euclid(map_size.1);
        end_locations.push((new_x, new_y));
    }
    end_locations
}

fn load_data() -> Vec<Robot> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|l| Robot::parse(&mut l.to_owned()))
        .collect()
}

struct Robot {
    start: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn parse(line: &mut String) -> Self {
        let mut digits = ["".to_owned(),"".to_owned(),"".to_owned(),"".to_owned()];
        let mut number_position = 0;
        for c in line.chars() {
            if c.is_digit(10) || c == '-' {
                digits[number_position].push(c);
            } else if c == ',' || c == ' ' {
                number_position += 1;
            }
        }
        Robot {
            start: (digits[0].parse().unwrap(), digits[1].parse().unwrap()),
            velocity: (digits[2].parse().unwrap(), digits[3].parse().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_robot() {
        let robot = Robot::parse(&mut "p=0,4 v=3,-3".to_owned());
        assert_eq!(robot.start.0, 0);
        assert_eq!(robot.start.1, 4);
        assert_eq!(robot.velocity.0, 3);
        assert_eq!(robot.velocity.1, -3);
    }
}

use advent_common::load_file_as_lines;
use std::cmp::min;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i64 {
    let lines = load_file_as_lines("./assets/input.txt");
    let mut total_square_feet = 0;
    for line in lines {
        let mut sides = line.split('x');
        let w = sides.next().unwrap().parse::<i64>().unwrap();
        let h = sides.next().unwrap().parse::<i64>().unwrap();
        let l = sides.next().unwrap().parse::<i64>().unwrap();
        let s1 = 2*w*h;
        let s2 = 2*h*l;
        let s3 = 2*w*l;
        let min = min(min(s1, s2), s3);
        total_square_feet += s1 + s2 + s3 + min;
    }
    total_square_feet
}

fn solution_2() -> i64 {
    0
}

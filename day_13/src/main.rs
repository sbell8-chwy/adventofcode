fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let mut best_bus = 0;
    let mut best_wait = data.0;
    data.1.iter()
        .map(|b| {
            let bus_wait = b - (data.0 % b);
            if bus_wait < best_wait {
                best_bus = *b;
                best_wait = bus_wait;
            }
        })
        .count();
    best_bus * best_wait
}

fn solution_2() -> u64 {
    let lines: Vec<String> = include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let mut busses: Vec<(usize, u64)> = lines[1].split(",")
        .enumerate()
        .filter_map(|(i, s)| s.parse::<u64>().map(|n| (i, n)).ok())
        .collect();
    busses.sort_unstable_by_key(|(_, bus)| *bus);
    busses.reverse();
    let mut time = 100000000000000;
    let mut busses_matched = 0;
    let mut step = 1;
    loop {
        match busses.iter()
            .position(|(delay, bus)| (time + *delay as u64) % *bus != 0)
            {
                None => break,
                Some(i) => {
                    if i > busses_matched {
                        busses_matched += 1;
                        step *= busses[i-1].1;
                    }
                },
            }
            time += step;
    }
    time
}

fn load_data() -> (usize, Vec<usize>) {
    let lines: Vec<String> = include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect();
    let time: usize = lines[0].parse().unwrap();
    let busses = lines[1].split(',')
        .filter(|&x| x != "x")
        .map(|s| s.parse().unwrap())
        .collect();
    (time, busses)
}

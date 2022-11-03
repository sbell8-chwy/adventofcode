fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let mut distance = 0;
    let mut depth = 0;
    for i in 0..data.len() {
        match data[i].direction.as_str() {
            "forward" => distance = distance + data[i].amount,
            "up" => depth = depth - data[i].amount,
            "down" => depth = depth + data[i].amount,
            _ => panic!("Failed Direction"),
        };
    }
    distance * depth
}

fn solution_2() -> usize {
    let data = load_data();
    let mut distance = 0;
    let mut depth = 0;
    let mut aim = 0;
    for i in 0..data.len() {
        match data[i].direction.as_str() {
            "forward" => {
                distance = distance + data[i].amount;
                depth = depth + (aim * data[i].amount);
            }
            "up" => aim = aim - data[i].amount,
            "down" => aim = aim + data[i].amount,
            _ => panic!("Failed Direction"),
        };
    }
    distance * depth
}

fn load_data() -> Vec<Field> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| parse_field(s))
        .collect()
}

fn parse_field(field_str: &str) -> Field {
    let parts: Vec<&str> = field_str.split(" ").collect();
    Field {
        direction: parts[0].to_string(),
        amount: parts[1].parse::<usize>().unwrap_or(0),
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field {
    pub direction: String,
    pub amount: usize,
}

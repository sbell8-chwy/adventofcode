fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u64 {
    let mut calibrations = load_data();
    let mut total = 0;
    'outer: for calibration in calibrations.iter_mut() {
        if calibration.validate() {
            total += calibration.test_value;
            continue;
        }
        while calibration.increment_operators() {
            if calibration.validate() {
                total += calibration.test_value;
                continue 'outer;
            }
        }
    }
    total
}

fn solution_2() -> u64 {
    0
}

fn load_data() -> Vec<Calibration> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| Calibration::new(s.into()))
        .collect()
}

struct Calibration {
    test_value: u64,
    measurements: Vec<u64>,
    operators: Vec<Operator>,
}

impl Calibration {
    fn new(line: String) -> Self {
        let halves: Vec<&str> = line.split(": ").collect();
        let test_value = halves[0].parse().unwrap();
        let measurements: Vec<u64> = halves[1].split(' ').map(|s| s.parse().unwrap()).collect();
        let operators = vec![Operator::PLUS; measurements.len() -1];
        Calibration {
            test_value,
            measurements,
            operators,
        }
    }

    fn increment_operators(&mut self) -> bool {
        for i in (0..self.operators.len()).rev() {
            match self.operators[i] {
                Operator::PLUS => {
                    self.operators[i] = Operator::MULTIPLY;
                    return true;
                }
                Operator::MULTIPLY => {
                    self.operators[i] = Operator::CONCAT;
                    return true;
                }
                Operator::CONCAT => {
                    self.operators[i] = Operator::PLUS;
                }
            }
        }
        false
    }

    fn validate(&self) -> bool {
        let mut total = self.measurements[0];
        for i in 0..self.operators.len() {
            match self.operators[i] {
                Operator::PLUS => total += self.measurements[i+1],
                Operator::MULTIPLY => total *= self.measurements[i+1],
                Operator::CONCAT => {
                    let total_str = total.to_string();
                    let next_val_str = self.measurements[i+1].to_string();
                    let total_str = total_str + &next_val_str;
                    total = total_str.parse::<u64>().unwrap();
                }
            }
        }
        total == self.test_value
    }
}

#[derive(Debug, Clone)]
enum Operator {
    PLUS,
    MULTIPLY,
    CONCAT, // solution 2 only
}

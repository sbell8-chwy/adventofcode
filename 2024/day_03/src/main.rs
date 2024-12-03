fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> i64 {
    let lines = load_data();
    let mut total = 0;
    for line in lines.iter() {
        total += compute_line(line.into());
    }
    total
}

fn solution_2() -> i64 {
    let lines = load_data();
    let mut total = 0;
    let mut enabled = true;
    for line in lines.iter() {
        let (temp_total, next_enabled) = compute_line_2(line.into(), enabled);
        enabled = next_enabled;
        total += temp_total;
    }
    total
}

fn compute_line_2(line: String, mut enabled: bool) -> (i64, bool) {
    let mut expect = "m";
    let mut flag_expect = "d";
    let mut first_digits = "".to_string();
    let mut second_digits = "".to_string();
    let mut total = 0;
    for c in line.chars() {
        // println!("expect: {}, first: {}, second: {}, total: {}, c: {}", expect, first_digits, second_digits, total, c);
        match c {
            'd' => {
                if flag_expect == "d" {
                    flag_expect = "o";
                    expect = "m";
                }
            }
            'o' => {
                if flag_expect == "o" && enabled {
                    flag_expect = "n";
                    expect = "m";
                } else if flag_expect == "o" && !enabled {
                    flag_expect = "(";
                    expect = "m";
                } else {
                    flag_expect = "d";
                    expect = "m";
                }
            }
            'n' => {
                if flag_expect == "n" {
                    flag_expect = "'";
                    expect = "m";
                } else {
                    flag_expect = "d";
                    expect = "m";
                }
            }
            '\'' => {
                if flag_expect == "'" {
                    flag_expect = "t";
                    expect = "m";
                } else {
                    flag_expect = "d";
                    expect = "m";
                }
            }
            't' => {
                if flag_expect == "t" {
                    flag_expect = "(";
                    expect = "m";
                } else {
                    flag_expect = "d";
                    expect = "m";
                }
            }
            'm' => {
                if expect == "m" {
                    expect = "u";
                    flag_expect = "d";
                }
            }
            'u' => {
                if expect == "u" {
                    expect = "l";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            'l' => {
                if expect == "l" {
                    expect = "(";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            '(' => {
                if expect == "(" {
                    expect = "d1";
                    flag_expect = "d";
                } else if flag_expect == "(" {
                    flag_expect = ")";
                    expect = "m";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            ',' => {
                if expect == "d," {
                    expect = "d2";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            ')' => {
                if expect == "d)" {
                    let first = first_digits.parse::<i64>().unwrap();
                    let second = second_digits.parse::<i64>().unwrap();
                    if enabled {
                        total += first * second;
                    } else {
                    }
                    expect = "m";
                    flag_expect = "d";
                } else if flag_expect == ")" {
                    enabled = !enabled;
                    flag_expect = "d";
                    expect = "m";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if expect == "d1" {
                    first_digits = c.to_string();
                    expect = "d,";
                    flag_expect = "d";
                } else if expect == "d," {
                    first_digits.push(c);
                    flag_expect = "d";
                } else if expect == "d2" {
                    second_digits = c.to_string();
                    expect = "d)";
                    flag_expect = "d";
                } else if expect == "d)" {
                    second_digits.push(c);
                    flag_expect = "d";
                } else {
                    expect = "m";
                    flag_expect = "d";
                }
            }
            _ => {
                expect = "m";
                flag_expect = "d";
            }
        };
    };
    (total, enabled)
}

fn compute_line(line: String) -> i64 {
    let mut expect = "m";
    let mut first_digits = "".to_string();
    let mut second_digits = "".to_string();
    let mut total = 0;
    for c in line.chars() {
        // println!("expect: {}, first: {}, second: {}, total: {}, c: {}", expect, first_digits, second_digits, total, c);
        match c {
            'm' => {
                if expect == "m" {
                    expect = "u";
                }
            }
            'u' => {
                if expect == "u" {
                    expect = "l";
                } else {
                    expect = "m";
                }
            }
            'l' => {
                if expect == "l" {
                    expect = "(";
                } else {
                    expect = "m";
                }
            }
            '(' => {
                if expect == "(" {
                    expect = "d1";
                } else {
                    expect = "m";
                }
            }
            ',' => {
                if expect == "d," {
                    expect = "d2";
                } else {
                    expect = "m";
                }
            }
            ')' => {
                if expect == "d)" {
                    let first = first_digits.parse::<i64>().unwrap();
                    let second = second_digits.parse::<i64>().unwrap();
                    total += first * second;
                    expect = "m";
                } else {
                    expect = "m";
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                if expect == "d1" {
                    first_digits = c.to_string();
                    expect = "d,";
                } else if expect == "d," {
                    first_digits.push(c);
                } else if expect == "d2" {
                    second_digits = c.to_string();
                    expect = "d)";
                } else if expect == "d)" {
                    second_digits.push(c);
                } else {
                    expect = "m";
                }
            }
            _ => expect = "m"
        };
    };
    total
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.into())
        .collect()
}

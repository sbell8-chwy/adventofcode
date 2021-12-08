use std::collections::HashSet;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let temp = load_data();
    let rules = generate_indexed_rules(&temp[0]);
    let messages = &temp[1];
    // println!("rules - {:?}", rules);
    // println!("messages - {:?}", messages);
    let matches = generate_matches(&rules);
    messages.iter().filter(|&m| matches.contains(m)).count()
}

fn solution_2() -> usize {
    0
}
/*
fn do_basic_replace(indexed_rules: &Vec<String>) -> Vec<String> {
    let ret_vals = vec!["".to_string(); indexed_rules.len()];
    let mut did_replace = false;
    loop {}
    ret_vals
}*/

fn generate_matches(indexed_rules: &Vec<String>) -> HashSet<String> {
    let mut matches: HashSet<String> = HashSet::new();
    let mut intermediates = vec![];
    intermediates.push(indexed_rules[0].to_string());
    loop {
        let mut temp: Vec<String> = vec![];
        for intermediate in intermediates.iter() {
            temp.extend(
                produce_next_string(&indexed_rules, intermediate)
                    .into_iter()
                    .filter(|s| {
                        let mut has_digit = false;
                        for c in s.chars() {
                            if c.is_numeric() {
                                has_digit = true;
                                break;
                            }
                        }
                        if !has_digit {
                            matches.insert(s.replace(" ", "").to_string());
                        }
                        has_digit
                    }),
            );
        }
        if temp.is_empty() {
            break;
        }
        intermediates = temp;
    }
    matches
}

fn produce_next_string(indexed_rules: &Vec<String>, current: &String) -> Vec<String> {
    let mut possibles: Vec<String> = vec![];
    for item in current.split_whitespace() {
        match item.parse::<usize>() {
            Ok(n) => {
                let mut temp: Vec<String> = vec![];
                match n {
                    8 => {
                        if possibles.is_empty() {
                            temp.push("42".to_string());
                            temp.push("42 42".to_string());
                            temp.push("42 42 42".to_string());
                            temp.push("42 42 42 42".to_string());
                        } else {
                            for possible in possibles.iter() {
                                temp.push(possible.to_owned() + " 42");
                                temp.push(possible.to_owned() + " 42 42");
                                temp.push(possible.to_owned() + " 42 42 42");
                                temp.push(possible.to_owned() + " 42 42 42 42");
                            }
                        }
                    }
                    11 => {
                        if possibles.is_empty() {
                            temp.push("42 31".to_string());
                            temp.push("42 42 31 31".to_string());
                            temp.push("42 42 42 31 31 31".to_string());
                            temp.push("42 42 42 42 31 31 31 31".to_string());
                        } else {
                            for possible in possibles.iter() {
                                temp.push(possible.to_owned() + " 42 31");
                                temp.push(possible.to_owned() + " 42 42 31 31");
                                temp.push(possible.to_owned() + " 42 42 42 31 31 31");
                                temp.push(possible.to_owned() + " 42 42 42 42 31 31 31 31");
                            }
                        }
                    }
                    _ => {
                        if possibles.is_empty() {
                            for next_part in indexed_rules[n].split(" | ") {
                                temp.push(next_part.to_string());
                            }
                        } else {
                            for possible in possibles.iter() {
                                for next_part in indexed_rules[n].split(" | ") {
                                    temp.push(possible.to_owned() + " " + next_part);
                                }
                            }
                        }
                    }
                }
                possibles = temp;
            }
            Err(_) => {
                let mut temp: Vec<String> = vec![];
                if possibles.is_empty() {
                    temp.push(item.to_owned());
                } else {
                    for possible in possibles.iter() {
                        temp.push(possible.to_owned() + " " + item);
                    }
                }
                possibles = temp;
            }
        }
    }
    // println!("{:?} => {:?}", current, possibles);
    possibles
}

fn generate_indexed_rules(rule_text: &Vec<String>) -> Vec<String> {
    let mut indexed_rules: Vec<String> = vec!["".to_string(); rule_text.len()];
    rule_text
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(": ").collect();
            indexed_rules[parts[0].parse::<usize>().unwrap()] =
                parts[1].replace("\"", "").to_string();
        })
        .count();
    indexed_rules
}

fn load_data() -> Vec<Vec<String>> {
    include_str!("../assets/input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.to_string()).collect())
        .collect()
}

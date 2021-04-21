fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u64 {
    load_data().iter().map(eval_expression).sum()
}

fn solution_2() -> u64 {
    load_data().iter().map(eval_expression_2).sum()
}

fn eval_expression(expression: &String) -> u64 {
    let mut output_queue: Vec<u64> = vec![];
    let mut operator_queue = vec![];
    for token in tokenize_expression(expression).iter() {
        match *token {
            "+" | "*" | "(" => operator_queue.push(token),
            ")" => {
                drop(operator_queue.pop());
                if !operator_queue.is_empty() && operator_queue[operator_queue.len() - 1] != &"(" {
                    let t1 = output_queue.pop().unwrap();
                    let t2 = output_queue.pop().unwrap();
                    let last_op = operator_queue.pop().unwrap();
                    match last_op {
                        &"+" => output_queue.push(t1 + t2),
                        &"*" => output_queue.push(t1 * t2),
                        _ => unreachable!(),
                    }
                }
            }
            _ => {
                let token = token.parse::<u64>().unwrap();
                if !operator_queue.is_empty() && operator_queue[operator_queue.len() - 1] != &"(" {
                    let last_num = output_queue.pop().unwrap();
                    let last_op = operator_queue.pop().unwrap();
                    match last_op {
                        &"+" => output_queue.push(last_num + token),
                        &"*" => output_queue.push(last_num * token),
                        _ => unreachable!(),
                    }
                } else {
                    output_queue.push(token);
                }
            }
        }
    }
    output_queue.pop().unwrap()
}

fn eval_expression_2(expression: &String) -> u64 {
    let mut output_queue: Vec<u64> = vec![];
    let mut operator_queue = vec![];
    for token in tokenize_expression(expression).iter() {
        // println!("{}", token);
        match *token {
            "+" | "*" | "(" => operator_queue.push(token),
            ")" => {
                let mut found_open = false;
                loop {
                    let last_op = operator_queue.pop();
                    // println!("Loop - {:?}", last_op);
                    match last_op {
                        Some(&"(") => {
                            if found_open {
                                operator_queue.push(&"(");
                                break;
                            }
                            found_open = true
                        }
                        Some(&"*") => {
                            if found_open {
                                operator_queue.push(&"*");
                                break;
                            }
                            let last_num = output_queue.pop().unwrap();
                            let last_num_2 = output_queue.pop().unwrap();
                            output_queue.push(last_num * last_num_2);
                        }
                        Some(&"+") => {
                            let last_num = output_queue.pop().unwrap();
                            let last_num_2 = output_queue.pop().unwrap();
                            output_queue.push(last_num + last_num_2);
                        }
                        None => break,
                        Some(_) => unreachable!(),
                    }
                    // println!("{:?}", output_queue);
                    // println!("{:?}", operator_queue);
                }
            }
            _ => {
                let token = token.parse::<u64>().unwrap();
                if !operator_queue.is_empty() && operator_queue[operator_queue.len() - 1] == &"+" {
                    operator_queue.pop();
                    let last_num = output_queue.pop().unwrap();
                    output_queue.push(last_num + token);
                } else {
                    output_queue.push(token)
                }
            }
        }
        // println!("{:?}", output_queue);
        // println!("{:?}", operator_queue);
    }
    output_queue.iter().fold(1, |c, x| c * x)
}

fn tokenize_expression(expression: &String) -> Vec<&str> {
    expression
        .split("")
        .filter(|&s| s != "" && s != " ")
        .collect()
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.to_string())
        .collect()
    // vec!["2 * 3 + (4 * 5)".to_string()]
    // vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()]
    // vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()]
    // vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()]
}

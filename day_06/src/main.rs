use std::collections::HashSet;

fn main() {
    let answers: Vec<usize> = load_data().iter()
        .map(count_answers_2)
        .collect();
    println!("{:?}", answers);
    println!("{:?}", answers.iter().sum::<usize>());
}

fn load_data() -> Vec<&'static str> {
    include_str!("../assets/input.txt").split("\n\n").collect()
}

fn count_answers(group: &&str) -> usize {
    let mut answers = HashSet::new();
    let _ = group.chars()
        .map(|c| {
            if c >= 'a' && c <= 'z' {
                answers.insert(c);
            }
            c
        })
        .collect::<Vec<char>>();
    answers.len()
}

fn count_answers_2(group: &&str) -> usize {
    let lines: Vec<&str> = group.lines().collect();
    let mut answers: HashSet<char> = lines[0].chars().collect();
    for line in lines.iter().skip(1) {
        let temp: HashSet<char> = line.chars().collect();
        answers = answers.intersection(&temp).copied().collect();
    }
    answers.len()
}

use std::cmp::Ordering;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u32 {
    let (rules, pages) = split_data(load_data());
    let mut total = 0;
    for page in pages.iter() {
        if is_valid_page(&rules, &page) {
            total += page[page.len() / 2];
        }
    }
    total
}

fn solution_2() -> u32 {
    let (rules, mut pages) = split_data(load_data());
    let mut total = 0;
    for page in pages.iter_mut() {
        if !is_valid_page(&rules, &page) {
            page.sort_by(|a,b| {
                if rules.contains(&(*a,*b)) {
                    Ordering::Less
                } else if rules.contains(&(*b,*a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            total += page[page.len() / 2];
        }
    }
    total
}

fn is_valid_page(rules: &Vec<(u32, u32)>, page: &Vec<u32>) -> bool {
    for i in 0..page.len() {
        for j in (i+1)..page.len() {
            if rules.contains(&(page[j], page[i])) {
                return false;
            }
        }
    }
    true
}

fn split_data(data: Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules = vec![];
    let mut pages = vec![];
    let mut found_empty_line = false;
    for line in data.iter() {
        if line == "" {
            found_empty_line = true;
        } else if found_empty_line {
            pages.push(line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect());
        } else {
            let rule_parts: Vec<u32> = line.split('|')
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            rules.push((rule_parts[0], rule_parts[1]));
        }
    }
    (rules, pages)
}

fn load_data() -> Vec<String> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.into())
        .collect()
}

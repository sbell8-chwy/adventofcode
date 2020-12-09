use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let data: Vec<(&str, Vec<(String, usize)>)> = load_data().into_iter()
        .map(parse_line)
        .collect();
    let mut bag_map: HashMap<String, Bag> = HashMap::new();
    for bag_data in data.iter() {
        bag_map.insert(bag_data.0.to_string(),
            Bag {
                name: bag_data.0.to_string(),
                contains: bag_data.1.to_owned(),
                contained_by: vec![],
            });
    }
    for bag_data in data.iter() {
        for item in &bag_data.1 {
            bag_map.get_mut(&item.0).unwrap().contained_by.push(bag_data.0.to_string());
        }
    }

    // part 1
    let mut bags = HashSet::new();
    let mut queue = vec!["shiny gold"];
    while !queue.is_empty() {
        let b = queue.pop().unwrap();
        match bag_map.get(b) {
            Some(bag) => {
                for item in bag.contained_by.iter() {
                    bags.insert(item);
                    queue.push(item);
                }
            },
            _ => (),
        }
    }
    println!("{:?}", bags.len());

    //part 2
    let mut bag_count = 0;
    let mut queue = vec![(1, "shiny gold")];
    while !queue.is_empty() {
        let b = queue.pop().unwrap();
        print!("{:?} => ", b);
        match bag_map.get(b.1) {
            Some(bag) => {
                for item in bag.contains.iter() {
                    print!("{} {}, ", (b.0 * item.1), item.0);
                    queue.push(((b.0 * item.1), &item.0));
                    bag_count += b.0 * item.1;
                }
            },
            _ => (),
        }
        print!("total: {}\n", bag_count);
    }
    println!("{}", bag_count);
}

fn load_data() -> Vec<&'static str> {
    include_str!("../assets/input.txt")
        .lines()
        .collect()
}

fn parse_line<'a>(line: &'a str) -> (&'a str, Vec<(String, usize)>) {
    let temp: Vec<&str> = line.split(" bags contain ").collect();
    let name = temp[0];
    let contains = Regex::new(r"bag[s]*[,.]").unwrap().split(temp[1])
        .into_iter()
        .filter(|s| s.trim() != "")
        .filter(|s| s.trim() != "no other")
        .map(|s| (s.trim()[2..].trim().to_string(), s.trim()[..2].trim().parse::<usize>().unwrap_or(0)))
        .collect();
    (name, contains)
}

#[derive(Debug, Default)]
struct Bag {
    pub name: String,
    pub contains: Vec<(String, usize)>,
    pub contained_by: Vec<String>,
}

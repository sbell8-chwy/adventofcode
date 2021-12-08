use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let data = load_data();
    let fields = data.0;
    let tickets = data.2;
    tickets.iter()
        .map(|t| t.iter().map(|&v| if !is_valid_for_any(v, &fields) { v } else { 0 }).sum::<usize>())
        .sum()
}

fn solution_2() -> usize {
    let data = load_data();
    let fields = data.0;
    let my_ticket = data.1;
    let tickets: Vec<Vec<usize>> = data.2;
    let tickets: Vec<&Vec<usize>> = tickets.iter()
        .filter(|t| {
            for v in t.iter() {
                if !is_valid_for_any(*v, &fields) {
                    return false;
                }
            }
            true
        })
        .collect();

    let mut matched_fields: HashMap<usize, Field> = HashMap::new();
    let mut field_map: HashMap<usize, HashSet<Field>> = HashMap::new();
    for i in 0..my_ticket.len() {
        field_map.insert(i, HashSet::from_iter(fields.iter().cloned()));
    }

    loop {
        for column in get_unfound_range(&matched_fields, my_ticket.len()) {
            let mut possible_fields: HashSet<Field> = HashSet::from_iter(fields.iter().cloned());
            for (_, value) in matched_fields.iter() {
                possible_fields.remove(value);
            }
            for f in fields.iter() {
                for t in tickets.iter() {
                    if !f.is_valid_value(t[column]) {
                        possible_fields.remove(f);
                    }
                }
            }
            if possible_fields.len() == 1 {
                matched_fields.insert(column, possible_fields.iter().next().unwrap().clone());
            }
        }
        if matched_fields.len() == fields.len() {
            break;
        }
    }
    let mut total = 1;
    for (k, v) in matched_fields.iter() {
        println!("checking field {}", v.name);
        if v.name.starts_with("departure") {
            total *= my_ticket[*k];
            println!("multiplying {}", my_ticket[*k]);
        }
    }
    total
}

fn get_unfound_range(matched_fields: &HashMap<usize, Field>, column_count: usize) -> Vec<usize> {
    let mut columns = vec![];
    for i in 0..column_count {
        if !matched_fields.contains_key(&i) {
            columns.push(i);
        }
    }
    columns
}

fn load_data() -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let sections:Vec<&str> = include_str!("../assets/input.txt")
        .split("\n\n")
        .collect();
    let fields = sections[0].split("\n")
        .map(|l| parse_field(l))
        .collect();

    let my_ticket = sections[1].split("\n").collect::<Vec<&str>>()[1].split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let tickets = sections[2].split("\n").skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.split(",").map(|s| s.parse::<usize>().unwrap()).collect())
        .collect();

    (fields, my_ticket, tickets)
}

fn is_valid_for_any(value: usize, fields: &Vec<Field>) -> bool {
    for f in fields {
        if f.is_valid_value(value) {
            return true;
        }
    }
    false
}

fn parse_field(field_str: &str) -> Field {
    let parts: Vec<&str> = field_str.split(": ").collect();
    let name = parts[0];
    let ranges = parts[1].split(" or ")
        .map(|r| {
            let numbers: Vec<&str> = r.split("-").collect();
            (numbers[0].parse::<usize>().unwrap(), numbers[1].parse::<usize>().unwrap())
        })
        .collect();
    Field { name: name.to_string(), ranges: ranges }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field {
    pub name: String,
    pub ranges: Vec<(usize, usize)>,
}

impl Field {
    fn is_valid_value(&self, value: usize) -> bool {
        for r in self.ranges.iter() {
            if value >= r.0 && value <= r.1 {
                return true;
            }
        }
        false
    }
}

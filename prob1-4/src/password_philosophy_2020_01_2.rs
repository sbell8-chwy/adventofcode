#[derive(Debug)]
struct PassData {
    pub min: usize,
    pub max: usize,
    pub needed: char,
    pub password: String,
}

pub fn solution() -> u32 {
    let pass_data = load_pass_data();
    println!("{}", pass_data.len());
    let mut count:u32 = 0;
    for data in pass_data.iter() {
        let minc = match data.password.chars().nth(data.min - 1) {
            Some(value) => value,
            None => ' ',
        };
        let maxc = match data.password.chars().nth(data.max - 1) {
            Some(value) => value,
            None => ' ',
        };
        if (minc == data.needed) != (maxc == data.needed) {
            println!("minc: {}, maxc: {}", minc, maxc);
            println!("Valid: {} or {} of {} in {}", data.min, data.max, data.needed, data.password);
            count += 1;
        }
        // let char_count = count_character(&data.needed, &data.password);
        // if char_count >= data.min && char_count <= data.max {
            // count += 1;
        // }
    }
    count
}

// fn count_character(needed: &String, password: &String) -> u32 {
    // let matches = password.matches(needed).count() as u32;
    // println!("found {} of {} in {}", matches, needed, password);
    // matches
// }

fn load_pass_data() -> Vec<PassData> {
    let br = include_str!("../assets/password_philosophy_2020_01_2.txt");
    br.lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let minmax_str = match iter.next() {
                Some(value) => value.to_string(),
                None => return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), },
            };
            // println!("{}", minmax_str);
            let mut minmax = minmax_str.split("-");
            let min = match minmax.next() {
                Some(value) => {
                    match value.parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Failed parsing min");
                            return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                        },
                    }
                },
                None => {
                    println!("None on min");
                    return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                },
            };
            let max = match minmax.next() {
                Some(value) => {
                    match value.parse::<usize>() {
                        Ok(v) => v,
                        Err(_) => {
                            println!("Failed parsing max");
                            return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                        },
                    }
                },
                None => {
                    println!("None on max");
                    return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                },
            };
            let needed = match iter.next() {
                Some(value) => value.chars().next().unwrap(),
                None => {
                    println!("None on needed");
                    return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                },
            };
            let password = match iter.next() {
                Some(value) => value.trim().to_string(),
                None => {
                    println!("None on password");
                    return PassData { min: 0, max: 0, needed: ' ', password: "".to_string(), };
                },
            };
            let ret_data = PassData {
                min: min,
                max: max,
                needed: needed,
                password: password,
            };
            ret_data
        })
        .collect()
}

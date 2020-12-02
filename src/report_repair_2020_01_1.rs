use std::io::{Error, ErrorKind};

pub fn solution() -> u32 {
    let numbers = load_numbers().unwrap();
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            for k in j..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }
            }
        }
    }
    0
}

fn load_numbers() -> Result<Vec<u32>, Error> {
    let br = include_str!("../assets/report_repair_2020_01_1.txt");
    br.lines()
        .map(|line| line.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
        .collect()
}

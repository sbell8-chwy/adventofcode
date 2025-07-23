use std::{fs::{self}, path::Path};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn load_file_as_lines<P>(filename: P) -> Vec<String> 
where 
    P: AsRef<Path>,
{
    fs::read_to_string(filename).expect("Failed to open file: {filename}")
        .lines()
        .map(|l| String::from(l))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

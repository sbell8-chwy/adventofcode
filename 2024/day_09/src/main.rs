fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let disk_map = load_data();
    let mut disk_blocks = create_disk_blocks(&disk_map);
    defrag(&mut disk_blocks);
    compute_disk(&disk_blocks)
}

fn solution_2() -> usize {
    let disk_map = load_data();
    let mut disk_blocks = create_disk_blocks(&disk_map);
    defrag_whole_file(&mut disk_blocks, &disk_map);
    compute_disk(&disk_blocks)
}

fn compute_disk(disk_blocks: &Vec<Option<usize>>) -> usize {
    let mut total = 0;
    for (i, disk_value) in disk_blocks.iter().enumerate() {
        match disk_value {
            Option::Some(x) => total += i * x,
            Option::None => (),
        }
    }
    total
}

fn defrag_whole_file(disk_blocks: &mut Vec<Option<usize>>, disk_map: &Vec<char>) {
    let mut free_space = vec![];
    let mut file_space = vec![];
    let mut next_index = 0;
    let mut file = true;
    for digit_char in disk_map.iter() {
        let digit = digit_char.to_digit(10).unwrap() as usize;
        if file {
            file_space.push((next_index, digit));
            file = false;
        } else {
            free_space.push((next_index, digit));
            file = true;
        }
        next_index += digit;
    }

    for file in file_space.iter().rev() {
        for free in free_space.iter_mut() {
            if file.1 <= free.1 {
                if free.0 >= file.0 {
                    break;
                }
                for i in 0..file.1 {
                    disk_blocks[i+free.0] = disk_blocks[i+file.0];
                    disk_blocks[i+file.0] = Option::None;
                }
                free.0 += file.1;
                free.1 -= file.1;
                break;
            }
        }
    }
}

fn defrag(disk_blocks: &mut Vec<Option<usize>>) {
    let mut front_index = 0;
    let mut tail_index = disk_blocks.len() -1;
    while front_index < tail_index {
        let front_value = disk_blocks.get(front_index).unwrap();
        let tail_value = disk_blocks.get(tail_index).unwrap();
        match front_value {
            Option::None => {
                match tail_value {
                    Option::None => {
                        tail_index -= 1;
                    },
                    Option::Some(_) => {
                        disk_blocks[front_index] = *tail_value;
                        disk_blocks[tail_index] = Option::None;
                        front_index += 1;
                        tail_index -= 1;
                    }
                }
            },
            Option::Some(_) => {
                front_index += 1;
            }
        }
    }
}

fn create_disk_blocks(disk_map: &Vec<char>) -> Vec<Option<usize>> {
    let mut disk_blocks = vec![];
    let mut file = true;
    let mut file_index = 0;
    for digit_char in disk_map.iter() {
        let digit = digit_char.to_digit(10).unwrap();
        for _ in 0..digit {
            if file {
                disk_blocks.push(Option::Some(file_index));
            } else {
                disk_blocks.push(Option::None);
            }
        }
        if file {
            file = false;
            file_index += 1;
        } else {
            file = true
        }
    }
    disk_blocks
}

fn load_data() -> Vec<char> {
    include_str!("../assets/input.txt")
        .trim_end()
        .chars()
        .collect()
}

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> usize {
    let (mut vec_1, mut vec_2) = load_data();
    vec_1.sort();
    vec_2.sort();
    let mut total = 0;
    for it in vec_1.iter().zip(vec_2.iter_mut()) {
        let (v1, v2) = it;
        if v1 > v2 {
            total += *v1 - *v2;
        } else {
            total += *v2 - *v1;
        }
    }
    total
}

fn solution_2() -> usize {
    let (mut vec_1, mut vec_2) = load_data();
    vec_1.sort();
    vec_2.sort();
    let mut similarity_score = 0;
    for i in vec_1.iter() {
        for j in vec_2.iter() {
            if i == j {
                similarity_score += j;
            }
        }
    }
    similarity_score
}

fn load_data() -> (Vec<usize>, Vec<usize>) {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| {
            let mut iter = s.split_whitespace();
            (iter.next().unwrap().parse::<usize>().unwrap(), iter.next().unwrap().parse::<usize>().unwrap())
        })
        .unzip()
}

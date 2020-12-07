fn main() {
    let mut seats: Vec<(usize, usize)> = load_data().iter()
        .map(find_seat)
        .collect();
    let mut seat_ids = seats.iter()
        .map(compute_seat_id)
        .collect::<Vec<usize>>();
    seat_ids.sort();
    seat_ids.dedup();
    find_missing_seat(&mut seats);
}

fn load_data() -> Vec<&'static str> {
    include_str!("../assets/input.txt").lines().collect()
}

fn find_seat(seat_code: &&str) -> (usize, usize) {
    let mut low_row = 0;
    let mut high_row = 127;
    let mut low_col = 0;
    let mut high_col = 7;
    seat_code.chars()
        .map(|c| {
            match c {
                'F' => high_row = (high_row + low_row) / 2,
                'B' => low_row = ((high_row + low_row) / 2) + 1,
                'L' => high_col = (high_col + low_col) / 2,
                'R' => low_col = ((high_col + low_col) / 2) + 1,
                _ => unreachable!(),
            }
        }).count();
    (low_row, low_col)
}

fn compute_seat_id((row, col): &(usize, usize)) -> usize {
    (row * 8) + col
}

fn find_missing_seat(seats: &mut Vec<(usize, usize)>) -> (usize, usize) {
    seats.sort();
    for (i, e) in seats.iter().enumerate() {
        if i+1 < seats.len() {
            let e2 = seats[i+1];
            let next = get_next_seat(e);
            if e2.0 != next.0 || e2.1 != next.1 {
                println!("{:?}, {:?}, {:?}, {:?}", e, e2, next, (next.0 * 8) + next.1);
            }
        }
    }
    (0, 0)
}

fn get_next_seat(seat: &(usize, usize)) -> (usize, usize) {
    if seat.1 == 7 {
        (seat.0+1, 0)
    } else {
        (seat.0, seat.1+1)
    }
}

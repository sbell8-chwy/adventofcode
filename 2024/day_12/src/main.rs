use std::collections::VecDeque;

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u32 {
    let mut farm = load_data();
    walk_farm(&mut farm);
    compute_fence(&farm)
}

fn solution_2() -> u32 {
    let mut farm = load_data();
    walk_farm(&mut farm);
    compute_fence_2(&farm)
}

fn compute_fence_2(farm: &Vec<Vec<Plot>>) -> u32 {
    let mut areas = vec![];
    areas.resize(farm.len() * farm[0].len(), (0, 0));
    for i in 0..farm.len() {
        for j in 0..farm[0].len() {
            let area_number = farm[i][j].field_number.unwrap();
            areas[area_number].0 += 1;
            // start of top fence
            if i == 0 || farm[i-1][j].crop != farm[i][j].crop {
                if j == 0
                || farm[i][j-1].crop != farm[i][j].crop
                || ((i > 0 && farm[i-1][j-1].crop == farm[i][j].crop) && farm[i][j-1].crop == farm[i][j].crop){
                    areas[area_number].1 += 1;
                }
            }
            // start left fence
            if j == 0 || farm[i][j-1].crop != farm[i][j].crop {
                if i == 0
                || farm[i-1][j].crop != farm[i][j].crop
                || ((j > 0 && farm[i-1][j-1].crop == farm[i][j].crop) && farm[i-1][j].crop == farm[i][j].crop){
                    areas[area_number].1 += 1;
                }
            }
            // start bottom fence
            if i == farm.len() - 1 || farm[i+1][j].crop != farm[i][j].crop {
                if j == 0
                || farm[i][j-1].crop != farm[i][j].crop
                || ((i < farm.len() - 1 && farm[i+1][j-1].crop == farm[i][j].crop) && farm[i][j-1].crop == farm[i][j].crop){
                    areas[area_number].1 += 1;
                }
            }
            // start right fence
            if j == farm[0].len() - 1 || farm[i][j+1].crop != farm[i][j].crop {
                if i == 0
                || farm[i-1][j].crop != farm[i][j].crop
                || ((j < farm[0].len() -1 && farm[i-1][j+1].crop == farm[i][j].crop) && farm[i-1][j].crop == farm[i][j].crop){
                    areas[area_number].1 += 1;
                }
            }
        }
    }

    let mut total = 0;
    for (area, fence) in areas {
        total += area * fence;
    }
    total
}

fn compute_fence(farm: &Vec<Vec<Plot>>) -> u32 {
    let mut areas = vec![];
    areas.resize(farm.len() * farm[0].len(), (0, 0));
    for i in 0..farm.len() {
        for j in 0..farm[0].len() {
            match farm[i][j].fence_count {
                Some(count) => {
                    areas[farm[i][j].field_number.unwrap()].0 += 1;
                    areas[farm[i][j].field_number.unwrap()].1 += count;
                }
                _ => (),
            }
        }
    }
    let mut total = 0;
    for (area, fence) in areas {
        total += area * fence;
    }
    total
}

fn walk_farm(farm: &mut Vec<Vec<Plot>>) {
    let mut field_count = 0;
    for i in 0..farm.len() {
        for j in 0..farm[0].len() {
            match farm[i][j].field_number {
                Option::Some(_) => (),
                Option::None => {
                    farm[i][j].field_number = Option::Some(field_count);
                    count_fences(farm, (i, j));
                    walk_area(farm, (i, j));
                    field_count += 1;
                }
            }
        }
    }
}

fn walk_area(farm: &mut Vec<Vec<Plot>>, plot_location: (usize, usize)) {
    let x = plot_location.0;
    let y = plot_location.1;
    let mut locations_to_check: VecDeque<(usize, usize)> = VecDeque::new();
    if plot_location.0 < farm.len() - 1 {
        locations_to_check.push_back((plot_location.0 + 1, plot_location.1));
    }
    if plot_location.1 < farm[0].len() - 1 {
        locations_to_check.push_back((plot_location.0, plot_location.1 + 1));
    }
    while let Some(loc) = locations_to_check.pop_front() {
        if farm[loc.0][loc.1].field_number == Option::None
            && farm[loc.0][loc.1].crop == farm[x][y].crop
        {
            farm[loc.0][loc.1].field_number = farm[x][y].field_number.clone();
            count_fences(farm, (loc.0, loc.1));
            if loc.0 < farm.len() - 1 {
                locations_to_check.push_back((loc.0 + 1, loc.1));
            }
            if loc.1 < farm[0].len() - 1 {
                locations_to_check.push_back((loc.0, loc.1 + 1));
            }
            if loc.0 > 0 {
                locations_to_check.push_back((loc.0 - 1, loc.1));
            }
            if loc.1 > 0 {
                locations_to_check.push_back((loc.0, loc.1 - 1));
            }
        }
    }
}

fn count_fences(farm: &mut Vec<Vec<Plot>>, plot_location: (usize, usize)) {
    let mut fence_count = 0;
    let x = plot_location.0;
    let y = plot_location.1;
    if x == 0 || farm[x - 1][y].crop != farm[x][y].crop {
        fence_count += 1;
    }
    if y == 0 || farm[x][y - 1].crop != farm[x][y].crop {
        fence_count += 1;
    }
    if x == farm.len() - 1 || farm[x + 1][y].crop != farm[x][y].crop {
        fence_count += 1;
    }
    if y == farm[0].len() - 1 || farm[x][y + 1].crop != farm[x][y].crop {
        fence_count += 1;
    }

    farm[x][y].fence_count = Some(fence_count);
}

fn load_data() -> Vec<Vec<Plot>> {
    include_str!("../assets/input.txt")
        .lines()
        .map(|s| s.chars().map(|c| c.into()).collect())
        .collect()
}

#[derive(Debug)]
struct Plot {
    crop: char,
    field_number: Option<usize>,
    fence_count: Option<u32>,
}

impl Into<Plot> for char {
    fn into(self) -> Plot {
        Plot {
            crop: self,
            field_number: Option::None,
            fence_count: Option::None,
        }
    }
}

fn main() {
    println!("Solution 1 => {}", solution_1());
    println!("Solution 2 => {}", solution_2());
}

fn solution_1() -> u64 {
    let tiles = get_matched_tiles();
    let mut product = 1;
    for t in tiles.iter() {
        println!("{:?}", t);
        if t.matched_sides() == 2 {
            product *= t.id;
        }
    }
    product
}

fn solution_2() -> usize {
    let tiles = get_matched_tiles();
    let _current_anchor = tiles
        .iter()
        .find(|t| t.top == None && t.left == None)
        .unwrap();
    // let mut whole_grid = vec![];
    0
}

fn get_matched_tiles() -> Vec<Tile> {
    let mut tiles = load_data().unwrap();
    for i in 1..tiles.len() - 1 {
        let (left, right) = tiles.split_at_mut(i);
        let t1 = left.last_mut().unwrap();
        for t2 in right.iter_mut() {
            t1.attempt_match(t2);
        }
    }
    tiles
}

fn parse_block(s: &str) -> anyhow::Result<Tile> {
    let mut i = s.lines();
    let id = i
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1][0..4]
        .parse::<u64>()
        .unwrap();
    let grid = i.map(|s| s.to_string()).collect();
    Ok(Tile {
        id,
        grid,
        top: None,
        bottom: None,
        left: None,
        right: None,
    })
}

fn load_data() -> anyhow::Result<Vec<Tile>> {
    include_str!("../assets/input_test.txt")
        .split("\n\n")
        .map(parse_block)
        .collect()
}

#[derive(PartialEq, Debug)]
struct Tile {
    pub id: u64,
    pub grid: Vec<String>,
    pub top: Option<u64>,
    pub bottom: Option<u64>,
    pub left: Option<u64>,
    pub right: Option<u64>,
}

impl Tile {
    fn attempt_match(&mut self, other: &mut Tile) {
        if self.top == None {
            let t = self.get_top();
            match attempt_match_line(t, other) {
                "top" => {
                    self.top = Some(other.id);
                    other.top = Some(self.id);
                    return;
                }
                "bottom" => {
                    self.top = Some(other.id);
                    other.bottom = Some(self.id);
                    return;
                }
                "left" => {
                    self.top = Some(other.id);
                    other.left = Some(self.id);
                    return;
                }
                "right" => {
                    self.top = Some(other.id);
                    other.right = Some(self.id);
                    return;
                }
                _ => (),
            }
        }
        if self.bottom == None {
            let t = self.get_bottom();
            match attempt_match_line(t, other) {
                "top" => {
                    self.bottom = Some(other.id);
                    other.top = Some(self.id);
                    return;
                }
                "bottom" => {
                    self.bottom = Some(other.id);
                    other.bottom = Some(self.id);
                    return;
                }
                "left" => {
                    self.bottom = Some(other.id);
                    other.left = Some(self.id);
                    return;
                }
                "right" => {
                    self.bottom = Some(other.id);
                    other.right = Some(self.id);
                    return;
                }
                _ => (),
            }
        }
        if self.left == None {
            let t = self.get_left();
            match attempt_match_line(t, other) {
                "top" => {
                    self.left = Some(other.id);
                    other.top = Some(self.id);
                    return;
                }
                "bottom" => {
                    self.left = Some(other.id);
                    other.bottom = Some(self.id);
                    return;
                }
                "left" => {
                    self.left = Some(other.id);
                    other.left = Some(self.id);
                    return;
                }
                "right" => {
                    self.left = Some(other.id);
                    other.right = Some(self.id);
                    return;
                }
                _ => (),
            }
        }
        if self.right == None {
            let t = self.get_right();
            match attempt_match_line(t, other) {
                "top" => {
                    self.right = Some(other.id);
                    other.top = Some(self.id);
                    return;
                }
                "bottom" => {
                    self.right = Some(other.id);
                    other.bottom = Some(self.id);
                    return;
                }
                "left" => {
                    self.right = Some(other.id);
                    other.left = Some(self.id);
                    return;
                }
                "right" => {
                    self.right = Some(other.id);
                    other.right = Some(self.id);
                    return;
                }
                _ => (),
            }
        }
    }

    fn matched_sides(&self) -> u64 {
        let mut i = 0;
        if self.top != None {
            i += 1;
        }
        if self.bottom != None {
            i += 1;
        }
        if self.left != None {
            i += 1;
        }
        if self.right != None {
            i += 1;
        }
        i
    }

    fn rotate(&mut self, times: u8) {
        let mut new_grid: Vec<String> = vec!["".to_string(); self.grid.len()];
        for _ in 0..times {
            for i in 0..self.grid.len() {
                for (j, c) in self.grid[i].chars().rev().enumerate() {
                    new_grid[j].push(c);
                }
            }
            let temp = self.top;
            self.top = self.right;
            self.right = self.bottom;
            self.bottom = self.left;
            self.left = temp;
        }
        self.grid = new_grid;
    }
    /*
        fn merge_right(&self, tiles: &mut Vec<Tile>) -> Vec<String> {
            let mut local_grid = self.grid.clone();
            local_grid.remove(0);
            local_grid.pop();
            local_grid = local_grid
                .iter()
                .map(|s| {
                    let mut s = s.to_owned();
                    s.remove(0);
                    s.pop();
                    s
                })
                .collect();
            match self.right {
                Some(n) => {
                    let mut new_vec: Vec<String> = vec![];
                    let next_tile: &mut Tile = tiles.iter_mut().find(|t| t.id == n).unwrap();
                    let mut rotations = 0;
                    if next_tile.top.unwrap_or(0u64) == self.id {
                        rotations = 1;
                    } else if next_tile.right.unwrap_or(0u64) == self.id {
                        rotations = 2;
                    } else if next_tile.bottom.unwrap_or(0u64) == self.id {
                        rotations = 3;
                    }
                    next_tile.rotate(rotations);
                    let next_grid = next_tile.merge_right(tiles);
                    for i in 0..local_grid.len() {
                        new_vec.push(local_grid[i].to_owned() + &next_grid[i]);
                    }
                    new_vec
                }
                None => local_grid,
            }
        }
    */

    fn get_top(&self) -> String {
        self.grid[0].to_string()
    }

    fn get_bottom(&self) -> String {
        self.grid[self.grid.len() - 1].to_string()
    }

    fn get_left(&self) -> String {
        let mut s = String::new();
        for g in self.grid.iter() {
            s.push(g.chars().next().unwrap());
        }
        s
    }

    fn get_right(&self) -> String {
        let mut s = String::new();
        for g in self.grid.iter() {
            s.push(g.chars().last().unwrap());
        }
        s
    }
}

fn attempt_match_line(self_line: String, other: &mut Tile) -> &str {
    if other.top == None {
        if self_line == other.get_top()
            || self_line == other.get_top().chars().rev().collect::<String>()
        {
            return "top";
        }
    }
    if other.bottom == None {
        if self_line == other.get_bottom()
            || self_line == other.get_bottom().chars().rev().collect::<String>()
        {
            return "bottom";
        }
    }
    if other.left == None {
        if self_line == other.get_left()
            || self_line == other.get_left().chars().rev().collect::<String>()
        {
            return "left";
        }
    }
    if other.right == None {
        if self_line == other.get_right()
            || self_line == other.get_right().chars().rev().collect::<String>()
        {
            return "right";
        }
    }
    ""
}

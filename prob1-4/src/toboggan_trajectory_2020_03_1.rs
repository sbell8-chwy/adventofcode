pub fn solution(stepx: usize, stepy: usize) -> u32 {
    let trees = load_trees();
    let mut tree_count = 0;
    let mut xpos = 0;
    for i in (0..trees.len()).step_by(stepy) {
        if trees[i].chars().nth(xpos % trees[i].len()) == Some('#') {
            tree_count += 1;
        }
        xpos += stepx;
    }
    tree_count
}

fn load_trees() -> Vec<String> {
    let br = include_str!("../assets/toboggan_trajectory_2020_03_1.txt");
    br.lines()
        .map(|line| line.to_string())
        .collect()
}

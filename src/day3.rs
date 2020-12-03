/// inner vec refers to each row (width-related)
/// outer ver refers to each column (height-related)
pub type Map = Vec<Vec<bool>>;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let mut trees = vec![];
    for row in input.lines() {
        let mut row_trees: Vec<bool> = vec![];
        for spot in row.chars() {
            if spot == '.' {
                row_trees.push(false)
            } else if spot == '#' {
                row_trees.push(true)
            } else {
                unimplemented!();
            }
        }
        trees.push(row_trees);
    }
    trees
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> usize {
    solve_scenario(input, 3, 1)
}

pub fn solve_scenario(input: &Map, dx: usize, dy: usize) -> usize {
    let mut collisions: usize = 0;
    let mut x_pos = 0usize;
    let width = input[0].len();
    let height = input.len();
    for i in 0..height {
        if i % dy == 0 {
            if input[i][x_pos] {
                collisions += 1;
            }
            x_pos += dx;
            x_pos %= width;
        }
    }
    collisions
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> usize {
   let scenarios: Vec<(usize, usize)> = vec![
       (1,1),
       (3,1),
       (5,1),
       (7,1),
       (1,2),
   ];
   let mut product = 1;
   for scenario in scenarios {
       product *= solve_scenario(input, scenario.0, scenario.1);
   }
   product
}
#[aoc_generator(day10)]
pub fn generate_input(input: &str) -> Vec<i32> {
    input.lines().map(|v| {
        v.parse().unwrap()
    }).collect()
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[i32]) -> usize {
    let mut sorted = input.to_owned();
    sorted.sort();
    let mut current_rating = sorted[0];
    let mut res: Vec<i32> = Vec::with_capacity(sorted.len());
    let len = sorted.len()-1;
    for i in 0usize..sorted.len() {
        if i < len {
            if sorted[i] > current_rating {
            } else {
                if sorted[i+1] > current_rating + 3 {
                    current_rating = sorted[i];
                    res.push(current_rating)
                }
            }       
        }
        
    }
}

//#[aoc(day, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    input.len() as i32
}
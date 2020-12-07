#[aoc_generator(day1)]
pub fn input_generator(inp: &str) -> Vec<u32> {
    inp.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(inp: &[u32]) -> u32 {
    for (i, a) in inp.iter().enumerate() {
        for b in inp[i..].iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!("Answer not found!");
}

#[aoc(day1, part1, loops)]
pub fn solve_part1_loop(inp: &[u32]) -> u32 {
    for i in 0..inp.len() {
        for j in i..inp.len() {
            if inp[i] + inp[j] == 2020 {
                return inp[i] * inp[j];
            }
        }
    }
    panic!("Answer not found!")
}

#[aoc(day1, part2)]
pub fn solve_part2(inp: &[u32]) -> u32 {
    for (i, a) in inp.iter().enumerate() {
        for (j, b) in inp[i..].iter().enumerate() {
            for c in inp[j..].iter() {
                if a + b == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!("Answer not found!");
}

#[aoc(day1, part2, loops)]
pub fn solve_part2_loop(inp: &[u32]) -> u32 {
    for i in 0..inp.len() {
        for j in i..inp.len() {
            for k in j..inp.len() {
                if inp[i] + inp[j] + inp[k] == 2020 {
                    return inp[i] * inp[j] * inp[k];
                }
            }
        }
    }
    panic!("Answer not found!")
}

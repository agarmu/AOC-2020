#[aoc_generator(day9)]
pub fn generate_input(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.parse::<i64>().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[i64]) -> i64 {
    let validity = input[25..]
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 25, v))
        .map(|(i, &v)| (is_valid(&input[i - 25..i], v), v));

    let lowest_non_valid = validity.filter(|(valid, _)| !*valid).next().unwrap();
    lowest_non_valid.1
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<i64>) -> i64 {
    let validity = input[25..]
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 25, v))
        .map(|(i, &v)| (is_valid(&input[i - 25..i], v), v));

    let lowest_non_valid = validity.filter(|(valid, _)| !*valid).next().unwrap();
    let (min, max) = find_contiguous_set(input, lowest_non_valid.1);
    //
    let section = &mut input.clone()[min..max];
    section.sort();
    section[0] + section[section.len() - 1]
}

pub fn find_contiguous_set(set: &[i64], value: i64) -> (usize, usize) {
    for i in 0..set.len() - 1 {
        let mut sum = set[i];
        for j in i + 1..set.len() {
            sum += set[j];
            if sum == value {
                return (i, j + 1);
            }
        }
    }
    panic!("Not found!");
}

pub fn is_valid(last_25: &[i64], value: i64) -> bool {
    for &a in last_25 {
        let should = value - a;
        for &b in &last_25[1..] {
            if b == should {
                return true;
            }
        }
    }
    false
}

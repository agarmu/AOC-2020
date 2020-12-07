type Person = String;
type Group = Vec<Person>;
use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn generate_input(inp: &str) -> Vec<Group> {
    inp.split("\n\n")
        .map(|group| group.lines().map(|person| (*person).to_owned()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(inp: &[Group]) -> i32 {
    inp.iter()
        .fold(0, |accumulator, group| accumulator + get_any_count(&group))
}

#[aoc(day6, part1, less-heap)]
pub fn solve_part1_less_heap(inp: &[Group]) -> i32 {
    inp.iter().fold(0, |accumulator, group| {
        accumulator + get_any_count_arr(&group)
    })
}

#[aoc(day6, part2)]
pub fn solve_part2(inp: &[Group]) -> i32 {
    inp.iter()
        .fold(0, |accumulator, group| accumulator + get_all_count(&group))
}

#[aoc(day6, part2, optimized)]
pub fn solve_part2_optimized(inp: &[Group]) -> i32 {
    inp.iter().fold(0, |accumulator, group| {
        accumulator + get_all_count_optimized(&group)
    })
}

#[aoc(day6, part2, less-heap)]
pub fn solve_part2_less_heap(inp: &[Group]) -> i32 {
    inp.iter().fold(0, |accumulator, group| {
        accumulator + get_all_count_arr(&group)
    })
}

pub fn get_any_count(group: &Group) -> i32 {
    get_keys(group).iter().fold(0, |acc, _| acc + 1)
}

pub fn get_all_count_optimized(group: &Group) -> i32 {
    let len = group.len() as i32;
    get_keys(group)
        .iter()
        .fold(0, |acc, (_k, v)| if *v == len { acc } else { 0 }) as i32
}

pub fn get_all_count(group: &Group) -> i32 {
    let len = group.len() as i32;
    get_keys_faster(group);
    get_keys(group)
        .iter()
        .filter(|(_k, v)| **v == len)
        .fold(0, |acc, _| acc + 1)
}

pub fn get_any_count_arr(group: &Group) -> i32 {
    let mut sum = 0;
    for v in &get_keys_faster(group) {
        if *v != 0 {
            sum += 1;
        }
    }
    sum
}

pub fn get_all_count_arr(group: &Group) -> i32 {
    let len = group.len() as i32;
    let mut sum = 0;
    for v in &get_keys_faster(group) {
        if *v == len {
            sum += 1;
        }
    }
    sum
}

pub fn get_keys(group: &Group) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for person in group {
        for question in person.chars() {
            let counter = map.entry(question).or_insert(1);
            *counter += 1;
        }
    }
    map
}

pub fn get_keys_faster(group: &Group) -> [i32; 26] {
    let mut res = [0; 26];
    for person in group {
        for question in person.bytes() {
            res[question as usize - 97] += 1;
        }
    }
    res
}

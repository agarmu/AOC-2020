use std::collections::HashMap;

pub type Bag = (String, String);
pub type ChildBag = (i32, Bag);
pub type Rule = (Bag, Vec<ChildBag>);
pub type ChildrenToParents = HashMap<String, Vec<String>>;
pub type ParentsToChildren = HashMap<Bag, Vec<ChildBag>>;

#[aoc_generator(day7)]
pub fn generate_input(inp: &str) -> String {
    inp.to_owned()
}

pub fn parse_input(inp: &str) -> Vec<Rule> {
    let mut rules = vec![];
    for line in inp.lines() {
        rules.push(parse_rule(line));
    }
    rules
}

pub fn get_children_to_parents(inp: &String) -> ChildrenToParents {
    let mut map: ChildrenToParents = HashMap::new();
    let rules = parse_input(inp);
    for (parent, children) in rules.into_iter() {
        let parent_repr = repr(&parent);
        for (_, child) in children.into_iter() {
            let parents = map.entry(repr(&child)).or_insert(vec![]);
            parents.push(parent_repr.clone());
        }
    }
    map
}

pub fn get_parents_to_children(inp: &String) -> ParentsToChildren {
    let mut map = HashMap::new();
    let rules = parse_input(inp);
    for rule in rules.into_iter() {
        map.insert(rule.0, rule.1);
    }
    map
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &String) -> usize {
    let input = get_children_to_parents(input);
    unique(get_parents_recursive(&input, &"shiny gold".to_owned())).len()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &String) -> i32 {
    get_child_count_recursive(
        &get_parents_to_children(input),
        &("shiny".to_owned(), "gold".to_owned()),
    )
}

fn parse_rule(rule: &str) -> Rule {
    let mut parts = rule.split("contain");
    let parent = parse_bag(parts.next().unwrap());
    let children = parts.next().unwrap();
    if children.contains("no other") {
        return (parent, vec![]);
    }
    let children_bags = children
        .split(',')
        .map(|child| parse_child_bag(child))
        .collect::<Vec<_>>();
    (parent, children_bags)
}

pub fn unique<T: Eq + std::hash::Hash>(vals: Vec<T>) -> HashMap<T, usize> {
    let mut map = HashMap::new();
    for val in vals.into_iter() {
        *map.entry(val).or_insert(0) += 1;
    }
    map
}

fn get_parents_recursive(map: &ChildrenToParents, value: &String) -> Vec<String> {
    let mut parents = vec![];
    if let Some(parents_raw) = map.get(value) {
        for parent in parents_raw {
            parents.push(parent.clone());
            parents.append(&mut get_parents_recursive(map, parent))
        }
    }
    parents
}

fn get_child_count_recursive(map: &ParentsToChildren, value: &Bag) -> i32 {
    let a = map.get(value).unwrap();
    a.iter()
        .map(|(count, bag)| count * (get_child_count_recursive(map, bag) + 1))
        .sum()
}

fn parse_bag(name: &str) -> Bag {
    let mut parts = name.split_ascii_whitespace().map(|x| (*x).to_owned());
    (parts.next().unwrap(), parts.next().unwrap())
}
fn parse_child_bag(name: &str) -> ChildBag {
    let mut parts = name.split_ascii_whitespace().map(|x| (*x).to_owned());
    let count = parts.next().unwrap();
    let count = match count.parse::<i32>() {
        Ok(v) => v,
        Err(_) => panic!("{} {}", name, count),
    };
    (count, (parts.next().unwrap(), parts.next().unwrap()))
}
fn repr(bag: &Bag) -> String {
    format!("{} {}", bag.0, bag.1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn random() {
        super::solve_part1(&include_str!("../input/2020/day7.txt").to_owned());
    }
}

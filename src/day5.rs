#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[String]) -> i32 {
    let mut id_list: Vec<_> = input.iter().map(|x| get_id(x.as_str())).collect();
    id_list.sort();
    id_list[id_list.len() - 1]
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[String]) -> i32 {
    let mut id_list: Vec<_> = input.iter().map(|x| get_id(x.as_str())).collect();
    id_list.sort();
    for i in 1..id_list.len() - 1 {
        let current = id_list[i];
        let next = id_list[i + 1];
        if (next - current) != 1 {
            //current is the one that has a gap after it, so add 1
            return current + 1;
        }
    }
    panic!("Not found");
}

pub fn get_id(input: &str) -> i32 {
    let split_data = input.chars().collect::<Vec<char>>();
    let mut row_min = 0;
    let mut row_max = 127;
    for row_data in &split_data[0..7] {
        match row_data {
            'F' => {
                row_max = (row_max + row_min) / 2;
            }
            'B' => {
                row_min = (row_min + row_max + 1) / 2;
            }
            _ => unimplemented!(),
        }
    }
    assert_eq!(row_min, row_max);
    let mut col_min = 0;
    let mut col_max = 7;
    for col_data in &split_data[7..10] {
        match col_data {
            'L' => {
                col_max = (col_min + col_max) / 2;
            }
            'R' => {
                col_min = (col_max + col_min + 1) / 2;
            }
            _ => unimplemented!(),
        }
    }
    assert_eq!(col_min, col_max);
    8 * row_min + col_min
}

#[cfg(test)]
pub mod tests {
    const CASES: [(&'static str, i32); 4] = [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];
    #[test]
    fn get_id_test() {
        for case in &CASES {
            let expected = super::get_id(case.0);
            let actual = case.1;
            assert_eq!(expected, actual);
        }
    }
    #[test]
    fn get_max_id() {
        let cases = CASES
            .iter()
            .map(|(val, _)| (*val).to_owned())
            .collect::<Vec<_>>();
        assert_eq!(super::solve_part1(&cases), 820);
    }
}

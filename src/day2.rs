pub struct Password {
    lower: u32,
    upper: u32,
    look: char,
    pwd: String
}

#[aoc_generator(day2)]
pub fn input_generator(inp: &str) -> Vec<Password> {
    inp.lines().map(|l| {
        //format: lower-upper char: pwd
        let mut phrases = l.split(' ');
        let mut lower_upper = phrases.next().unwrap()
            .split('-').map(|b| b.parse::<u32>().unwrap());
        let lower = lower_upper.next().unwrap();
        let upper = lower_upper.next().unwrap();
        let look: char = phrases.next().unwrap().split(':')
            .next().unwrap().chars().next().unwrap();
        let pwd = phrases.next().unwrap().to_owned();
        
        Password {lower, upper, look, pwd}
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(inp: &[Password]) -> u32 {
    let mut ans: u32 = 0;
    for a in inp.iter() {
        let mut count: u32 = 0;
        for c in a.pwd.chars() {
            if c == a.look {
                count+= 1;
            }
        }
        if a.lower <= count && count <= a.upper {
            ans += 1;
        } 
    }
    ans
}

#[aoc(day2, part2)]
pub fn solve_part2(inp: &[Password]) -> u32 {
    let mut ans: u32 = 0;
    for a in inp.iter() {
        let pwd_chars = a.pwd.chars().collect::<Vec<char>>();
        if (pwd_chars[(a.lower as usize)-1] == a.look) ^ (pwd_chars[(a.upper as usize)-1] == a.look) {
            ans += 1;
        }
    }
    ans
}
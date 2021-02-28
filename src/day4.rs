pub struct RawPassport {
    pub byr: String,         //Birth year
    pub iyr: String,         //Passport issue year
    pub eyr: String,         //Passport expiration year
    pub hgt: String,         //Height (cm or in)
    pub hcl: String,         //Hair color
    pub ecl: String,         //Eye color
    pub pid: String,         //Passport ID
    pub cid: Option<String>, //Country ID
}

impl RawPassport {
    pub fn parse(input: &[&str]) -> Result<Self, String> {
        let present: &[String] = &[];
        Err(String::from("Hi"))
    }
}

pub struct Passport {
    pub complete: bool,
    pub byr: Option<u16>,    //Birth year
    pub iyr: Option<u16>,    //Passport issue year
    pub eyr: Option<u16>,    //Passport expiration year
    pub hgt: Option<Height>, //Height (cm or in)
    pub hcl: Option<Color>,  //Hair color
    pub ecl: Option<Color>,  //Eye color
    pub pid: Option<u32>,    //Passport ID
    pub cid: Option<String>, //Country ID
}

pub enum Height {
    Inches(i32),
    Centimeters(i32),
}

pub struct Color(u8, u8, u8);

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> String {
    input.to_owned()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &String) -> i32 {
    input
        .split("\n\n")
        .map(
            |x| match RawPassport::parse(&x.split_ascii_whitespace().collect::<Vec<_>>()) {
                Ok(_) => true,
                Err(_) => false,
            },
        )
        .fold(0i32, |acc, v| if v { acc + 1 } else { acc })
}

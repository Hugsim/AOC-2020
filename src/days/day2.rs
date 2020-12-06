use crate::util::*;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct PasswordPolicy {
    range: RangeInclusive<i32>,
    letter: char,
    password: String,
}

impl FromStr for PasswordPolicy {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .split(|c| ['-', ':', ' '].contains(&c))
            .filter(|s| s != &"")
            .collect();

        let start: i32 = coords[0].parse()?;
        let end: i32 = coords[1].parse::<i32>()?;

        let letter: char = coords[2]
            .parse()
            .expect("Shouldn't happen! Failed parsing length 1 str into char");

        Ok(PasswordPolicy {
            range: start..=end,
            letter,
            password: String::from(coords[3]),
        })
    }
}

impl PasswordPolicy {
    pub fn validate_part1(self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.letter).count();
        self.range.contains(&(count as i32))
    }

    pub fn validate_part2(self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        (chars[*self.range.start() as usize - 1] == self.letter)
            ^ (chars[*self.range.end() as usize - 1] == self.letter)
    }
}

pub fn eval() {
    let contents: Vec<PasswordPolicy> = read_file_to_vec("src/days/input/2");
    let num = contents
        .into_iter()
        .map(|pwp| pwp.validate_part1())
        .filter(|&b| b)
        .count();
    sprint("Part 1: ");
    sprint(num);

    let contents: Vec<PasswordPolicy> = read_file_to_vec("src/days/input/2");
    let num = contents
        .into_iter()
        .map(|pwp| pwp.validate_part2())
        .filter(|&b| b)
        .count();
    sprint("Part 2: ");
    sprint(num);
}

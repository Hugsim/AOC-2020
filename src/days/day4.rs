use crate::util::*;
use std::str::FromStr;

pub fn eval() {
    let input = read_file_to_string("src/days/input/4");
    let input = input.split("\n\n").map(|s| {
        s.split('\n').collect::<Vec<&str>>()[..].join(" ")
    });

    let mut num_valid = 0;
    for s in input {
        match s.parse::<PassportData>() {
            Err(_) => {},
            Ok(_)  => num_valid += 1,
        }
    }

    sprint(num_valid);
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PassportData {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl PassportData {
    pub fn empty() -> PassportData {
        PassportData {
            byr: "".to_string(),
            iyr: "".to_string(),
            eyr: "".to_string(),
            hgt: "".to_string(),
            hcl: "".to_string(),
            ecl: "".to_string(),
            pid: "".to_string(),
            cid: "".to_string(),
        }
    }

    pub fn valid(&self) -> bool {
        let byr_valid = !self.byr.is_empty() && match self.byr.parse::<i32>() {
            Ok(v)  => v >= 1920 && v <= 2002,
            Err(_) => false,
        };
        let iyr_valid = !self.iyr.is_empty() && match self.iyr.parse::<i32>() {
            Ok(v)  => v >= 2010 && v <= 2020,
            Err(_) => false,
        };
        let eyr_valid = !self.eyr.is_empty() && match self.eyr.parse::<i32>() {
            Ok(v)  => v >= 2020 && v <= 2030,
            Err(_) => false,
        };
        let hgt_valid = !self.hgt.is_empty() && match &self.hgt[self.hgt.len()-2..] {
            "cm" => {
                let hgt = if let Ok(v) = self.hgt[0..3].parse::<i32>() {
                    v
                } else { return false; };
                hgt >= 150 && hgt <= 193
            },
            "in" => {
                let hgt = if let Ok(v) = self.hgt[0..2].parse::<i32>() {
                    v
                } else { return false; };
                hgt >= 59 && hgt <= 76
            },
            _ => false,
        };
        let hcl_valid = !self.hcl.is_empty() && match &self.hcl[0..1] {
            "#" => {
                let hcl = &self.hcl[1..];
                hcl.chars().all(|c| c.is_ascii_hexdigit())
            },
            _ => false,
        };
        let ecl_valid = !self.ecl.is_empty() && matches!(&self.ecl[..],
              "amb" 
            | "blu" 
            | "brn"
            | "gry"
            | "grn"
            | "hzl"
            | "oth"
        );
        let pid_valid = !self.pid.is_empty() && self.pid.len() == 9 && self.pid.chars().all(|c| c.is_ascii_digit());
        let cid_valid = true;

           byr_valid
        && iyr_valid
        && eyr_valid
        && hgt_valid
        && hcl_valid
        && ecl_valid
        && pid_valid
        && cid_valid
    }
}

impl FromStr for PassportData {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<(String, String)> = s.split(' ').map(|s| {
            let s = 
            s.split(':')
             .map(String::from)
             .collect::<Vec<String>>();

            (s[0].clone(), s[1].clone())
        }).collect();

        let mut res = PassportData::empty();
        for (k, v) in data {
            match &k[..] {
                "byr" => res.byr = v,
                "iyr" => res.iyr = v,
                "eyr" => res.eyr = v,
                "hgt" => res.hgt = v,
                "hcl" => res.hcl = v,
                "ecl" => res.ecl = v,
                "pid" => res.pid = v,
                "cid" => res.cid = v,
                _ => unreachable!(),
            }
        }
        
        if res.valid() {
            Ok(res)
        } else {
            Err(())
        }
    }
}

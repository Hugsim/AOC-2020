use crate::util::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Mask([Option<bool>; 36]);

impl Mask {
    pub fn new() -> Self {
        Mask([None; 36])
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = [None; 36];

        for (i, c) in s.chars().enumerate() {
            res[i] = match c {
                'X' => None,
                '0' => Some(false),
                '1' => Some(true),
                _ => unreachable!(),
            }
        }

        Ok(Mask(res))
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Update(Mask),
    Set(u64, u64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split(" = ").collect();
        let instr = match &s[0][0..3] {
            "mem" => {
                let number_len = s[0].len() - 5;
                let index = s[0][4..4 + number_len].parse::<u64>().unwrap();
                let value = s[1].parse::<u64>().unwrap();
                Instruction::Set(index, value)
            }
            "mas" => {
                let mask = s[1].parse::<Mask>().unwrap();
                Instruction::Update(mask)
            }
            _ => unreachable!(),
        };

        Ok(instr)
    }
}

#[derive(Clone, Debug)]
struct DockingParameters {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl DockingParameters {
    pub fn new() -> DockingParameters {
        DockingParameters {
            mask: Mask::new(),
            memory: HashMap::new(),
        }
    }

    pub fn set1(&mut self, idx: u64, val: u64) {
        let mut masked_val = 0;

        for (i, mask_bit) in self.mask.0.iter().enumerate() {
            if let Some(b) = mask_bit {
                masked_val = (masked_val << 1) | *b as u64;
            } else {
                masked_val = (masked_val << 1) | ((val >> (35 - i)) & 1);
            }
        }

        self.memory.insert(idx, masked_val);
    }

    pub fn set2(&mut self, idx: u64, val: u64) {
        let mut masked_addr = 0;
        let mut floating_bits = Vec::new();

        for (i, mask_bit) in self.mask.0.iter().enumerate() {
            if let Some(b) = mask_bit {
                if *b {
                    masked_addr = (masked_addr << 1) | 1;
                } else {
                    masked_addr = (masked_addr << 1) | ((idx >> (35 - i)) & 1);
                }
            } else {
                floating_bits.push(i);
                masked_addr <<= 1;
            }
        }

        let mut possible_addrs = vec![masked_addr];
        for bit in floating_bits {
            let mut new_vec = Vec::new();
            for addr in possible_addrs {
                new_vec.push(addr | (1 << (35 - bit)));
                new_vec.push(addr & !(1 << (35 - bit)));
            }
            possible_addrs = new_vec;
        }
        for addr in possible_addrs {
            self.memory.insert(addr, val);
        }
    }
}

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents = read_file_to_vec::<Instruction>("src/days/input/14");

    let mut dp = DockingParameters::new();

    for &instr in &contents {
        match instr {
            Instruction::Update(mask) => {
                dp.mask = mask;
            }
            Instruction::Set(idx, val) => {
                dp.set1(idx, val);
            }
        }
    }

    let mut sol1 = 0;
    for (_, v) in dp.memory {
        sol1 += v;
    }

    let mut dp = DockingParameters::new();

    for instr in contents {
        match instr {
            Instruction::Update(mask) => {
                dp.mask = mask;
            }
            Instruction::Set(idx, val) => {
                dp.set2(idx, val);
            }
        }
    }

    let mut sol2 = 0;
    for (_, v) in dp.memory {
        sol2 += v;
    }

    (Some(sol1 as i64), Some(sol2 as i64))
}

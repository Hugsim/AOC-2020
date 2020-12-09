use std::collections::HashSet;
use std::str::FromStr;

use crate::util::*;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Program {
    program: Vec<Instruction>,
    program_counter: i64,
    acc: i64,
}

impl Program {
    pub fn exec(&mut self) {
        match self.program[self.program_counter as usize] {
            Instruction::NOP(_) => self.program_counter += 1,
            Instruction::ACC(a) => {
                self.acc += a;
                self.program_counter += 1;
            }
            Instruction::JMP(j) => self.program_counter += j,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        let s: Vec<&str> = s.split(' ').collect();
        let arg = s[1].parse::<i64>().unwrap();

        match s[0] {
            "nop" => Ok(Instruction::NOP(arg)),
            "acc" => Ok(Instruction::ACC(arg)),
            "jmp" => Ok(Instruction::JMP(arg)),
            _ => Err(()),
        }
    }
}

pub fn solve() -> (Option<i64>, Option<i64>) {
    let instrs = read_file_to_vec::<Instruction>("src/days/input/8");

    let mut executed_instrs = HashSet::<i64>::new();

    let program = Program {
        program: instrs,
        program_counter: 0,
        acc: 0,
    };

    // Solves part 1
    let mut prog1 = program.clone();
    loop {
        if !executed_instrs.insert(prog1.program_counter) {
            break;
        }
        prog1.exec();
    }

    let mut possible_programs: Vec<Program> = vec![program.clone()];
    for (i, instr) in program.program.iter().enumerate() {
        match instr {
            Instruction::ACC(_) => continue,
            Instruction::JMP(j) => {
                let mut new_program = program.clone();
                new_program.program.remove(i);
                new_program.program.insert(i, Instruction::NOP(*j));
                possible_programs.push(new_program);
            }
            Instruction::NOP(n) => {
                let mut new_program = program.clone();
                new_program.program.remove(i);
                new_program.program.insert(i, Instruction::JMP(*n));
                possible_programs.push(new_program);
            }
        }
    }

    let mut sol2 = 0;
    for mut prog in possible_programs.clone() {
        let mut executed_instrs = HashSet::new();
        loop {
            if !executed_instrs.insert(prog.program_counter) {
                break;
            }
            prog.exec();
            if prog.program_counter >= prog.program.len() as i64 {
                sol2 = prog.acc;
                break;
            }
        }
    }

    (Some(prog1.acc), Some(sol2))
}

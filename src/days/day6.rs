use crate::util::*;

use std::collections::HashSet;

pub fn eval() {
    let contents = read_file_to_string("src/days/input/6");
    let result1: usize = contents
        .split("\n\n")
        .map(|s| {
            calculate_answer1(
                &s.split('\n')
                    .map(|s| s.chars().collect())
                    .collect::<Vec<Vec<char>>>()[..],
            )
        })
        .sum();
    let result2: usize = contents
        .split("\n\n")
        .map(|s| {
            calculate_answer2(
                &s.split('\n')
                    .map(|s| s.chars().collect())
                    .collect::<Vec<Vec<char>>>()[..],
            )
        })
        .sum();

    sprint("Part 1:");
    sprint(result1);
    sprint("Part 2:");
    sprint(result2);
}

fn calculate_answer1(vecs: &[Vec<char>]) -> usize {
    let mut set = HashSet::new();
    for vec in vecs {
        for c in vec {
            set.insert(c);
        }
    }
    set.len()
}

fn calculate_answer2(vecs: &[Vec<char>]) -> usize {
    vecs.iter()
        .map(|cs| cs.iter().map(|c| *c).collect::<HashSet<char>>())
        .fold_first(|acc, set| &acc & &set)
        .unwrap()
        .len()
}

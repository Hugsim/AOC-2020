use crate::util::*;

use std::collections::HashSet;

pub fn solve() -> (Option<i64>, Option<i64>) {
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

    (Some(result1 as i64), Some(result2 as i64))
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
        .map(|cs| cs.iter().copied().collect::<HashSet<char>>())
        .fold_first(|acc, set| &acc & &set)
        .unwrap()
        .len()
}

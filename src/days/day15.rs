use crate::util::*;
use std::collections::HashMap;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let numbers = read_file_to_string("src/days/input/15");
    let numbers = numbers
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();

    (Some(solve1(&mut numbers.clone())), Some(solve2(numbers)))
}

fn solve1(numbers: &mut Vec<i64>) -> i64 {
    while numbers.len() < 2020 {
        let last_num = numbers[numbers.len() - 1];
        if !numbers.contains(&last_num) {
            numbers.push(0);
            continue;
        }
        let mut last_index = numbers.len() - 1;
        for (i, &num) in numbers.iter().enumerate().take(numbers.len() - 1) {
            if num == last_num {
                last_index = i;
            }
        }
        numbers.push((numbers.len() as i64 - 1) - last_index as i64);
    }

    numbers.pop().unwrap()
}

fn solve2(numbers: Vec<i64>) -> i64 {
    let mut seen_numbers: HashMap<i64, i64> = numbers
        .iter()
        .enumerate()
        .map(|(v, &k)| (k, v as i64 + 1))
        .collect();
    let mut next_number = numbers[numbers.len() - 1];

    for curr_idx in (numbers.len() as i64)..30000000 {
        if let Some(last_idx) = seen_numbers.get(&next_number) {
            let diff = curr_idx - last_idx;
            seen_numbers.insert(next_number, curr_idx);
            next_number = diff;
        } else {
            seen_numbers.insert(next_number, curr_idx);
            next_number = 0;
        }
    }

    next_number
}

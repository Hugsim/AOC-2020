use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents: Vec<i64> = read_file_to_vec("src/days/input/9");
    let mut sol1 = 0;
    for (i, x) in contents.iter().enumerate() {
        let mut sums: Vec<i64> = product(&contents[(i as i64 - 25).max(0) as usize..i])
            .iter()
            .map(|(a, b)| a + b)
            .collect();
        sums.sort_unstable();
        sums.dedup();
        if !sums.contains(x) {
            sol1 = *x;
        }
    }

    let mut sol2 = 0;
    for (i, x) in contents.iter().enumerate() {
        let mut acc = *x;
        for y in &contents[(i + 1)..contents.len()] {
            acc += y;
            if acc == sol1 {
                sol2 = x + y;
            }
        }
    }

    (Some(sol1), Some(sol2))
}

fn product(s1: &[i64]) -> Vec<(i64, i64)> {
    let mut res = Vec::new();

    for x in s1 {
        for y in s1 {
            res.push((*x, *y));
        }
    }

    res
}

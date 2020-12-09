use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents: Vec<i64> = read_file_to_vec("src/days/input/9");
    
    for (i, x) in contents.iter().enumerate() {
        let pairs = product(&contents[(i-25).max(0)..i]);
    }



    unimplemented!();
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

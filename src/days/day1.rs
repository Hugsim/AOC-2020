use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let contents = read_file_to_vec::<i32>("src/days/input/1");

    let mut res1 = 0;
    for x in &contents {
        for y in &contents {
            if x + y == 2020 {
                res1 = x * y;
            }
        }
    }

    let mut res2 = 0;
    for x in &contents {
        for y in &contents {
            for z in &contents {
                if x + y + z == 2020 {
                    res2 = x * y * z;
                }
            }
        }
    }

    (Some(res1 as i64), Some(res2 as i64))
}

use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let mut data = read_file_to_vec::<i64>("src/days/input/10");
    data.push(0);
    data.sort_unstable();
    data.push(data[data.len() - 1] + 3);

    let mut diff1 = 0;
    let mut diff3 = 0;
    let mut diffs: Vec<i64> = Vec::new();
    for w in data.windows(2) {
        let diff = w[1] - w[0];
        if diff == 1 {
            diff1 += 1;
            diffs.push(1);
        } else if diff == 3 {
            diff3 += 1;
            diffs.push(3);
        }
    }
    let sol1 = diff1 * diff3;

    let diffs: Vec<i64> = diffs
        .split(|d| *d == 3)
        .map(|s| s.to_vec())
        .filter(|ds| ds.len() >= 2)
        .map(|ds| match ds.len() {
            2 => 2,
            3 => 4,
            4 => 7,
            _ => unreachable!(),
        })
        .collect();

    let sol2 = diffs.into_iter().fold_first(|x, acc| x * acc).unwrap();

    (Some(sol1), Some(sol2))
}

use crate::util::*;

pub fn solve() -> (Option<i64>, Option<i64>) {
    let input = read_file_to_vec::<String>("src/days/input/13");
    let time = input[0].parse::<i64>().unwrap();
    let buses = input[1]
        .split(',')
        .enumerate()
        .filter(|(_, b)| b != &"x")
        .map(|(i, b)| (i, b.parse::<i64>().unwrap()))
        .collect::<Vec<(usize, i64)>>();

    let mut times: Vec<(i64, i64, i64)> = buses
        .iter()
        .map(|(i, b)| (*b, (time / b + 1) * b - time, *i as i64))
        .collect();

    let min_time: &(i64, i64, i64) = times
        .iter()
        .min_by(|(_, x, _), (_, y, _)| x.cmp(y))
        .unwrap();
    let sol1 = min_time.0 * min_time.1;

    // times.sort_by(|(x, _, _), (y, _, _)| y.cmp(x));

    // Bruteforce loop, not feasible to
    // let mut time = 0;
    // let sol2 = loop {
    //     if times.iter().all(|(b, _, i)| (time + *i) % b == 0) {
    //         //println!("Found time: {}", time);
    //         break time;
    //     } else {
    //         //println!("Time {} didn't work", time);
    //     }
    //     time += 1;
    // };

    dprint(&times);

    let input_to_website = times
        .iter()
        .enumerate()
        .map(|(i, (b, _, i2))| {
            format!(
                "{}{} {}-1 {}\n",
                "0 ".repeat(i),
                b,
                "0 ".repeat(times.len() - 1 - i),
                i2
            )
        })
        .collect::<String>();
    sprint(input_to_website);
    // Gives back the following:
    /*
        13 0 0 0 0 0 0 0 0 -1 0
        0 37 0 0 0 0 0 0 0 -1 7
        0 0 461 0 0 0 0 0 0 -1 13
        0 0 0 17 0 0 0 0 0 -1 27
        0 0 0 0 19 0 0 0 0 -1 32
        0 0 0 0 0 29 0 0 0 -1 42
        0 0 0 0 0 0 739 0 0 -1 44
        0 0 0 0 0 0 0 41 0 -1 54
        0 0 0 0 0 0 0 0 23 -1 67
    */
    // Input this into http://www.numbertheory.org/php/axb.html to get the solution:
    let sol2 = 552612234243498;

    (Some(sol1), Some(sol2))
}

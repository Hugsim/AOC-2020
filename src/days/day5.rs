use crate::util::*;

pub fn eval() {
    let contents = read_file_to_vec::<String>("src/days/input/5");
    let mut contents: Vec<u32> = contents
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'F' => 0,
                    'B' => 1,
                    'L' => 0,
                    'R' => 1,
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>()
        })
        .map(|a| vec_to_u32(&a[..]))
        .collect();
    contents.sort_unstable();
    sprint("Part 1: ");
    let max = &contents.iter().max().unwrap().clone();
    dprint(max);

    sprint("Part 2: ");
    let mut looking = false;
    let mut start = 0;
    loop {
        if contents.contains(&start) {
            break;
        }
        start += 1;
    }
    for i in start..*max {
        if !contents.contains(&i) {
            sprint(i)
        }
    }
}

fn vec_to_u32(vec: &[u8]) -> u32 {
    let mut acc = 0;
    for x in vec {
        acc <<= 1;
        acc += *x as u32;
    }
    acc
}

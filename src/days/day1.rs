use crate::util::*;

pub fn eval() {
    let contents = read_file_to_vec::<i32>("src/days/day1/input");

    for x in &contents {
        for y in &contents {
            for z in &contents {
                if x + y + z == 2020 {
                    println!("Found! {}, {} and {} with product {}.", x, y, z, x * y * z);
                }
            }
        }
    }
}
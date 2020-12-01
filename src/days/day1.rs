use std::fs;

pub fn eval() {
    let contents = fs::read_to_string("src/days/day1/input").expect("Couldn't open file");

    let contents = contents[..].split('\n').into_iter().map(|x| x.parse::<i32>().expect("Couldn't parse value in file")).collect::<Vec<i32>>();

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
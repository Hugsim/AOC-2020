use std::fs;

pub fn read_file_to_string(file: &str) -> String {
    fs::read_to_string(file).expect(format!("Couldn't open {}", file))
}

pub fn read_file_to_vec<T>(file: &str) -> Vec<T> 
where 
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    read_file_to_string(file).split('\n').into_iter().map(|x| x.parse::<T>().expect("Couldn't parse value in file")).collect::<Vec<T>>()
}
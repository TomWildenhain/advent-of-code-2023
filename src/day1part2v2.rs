use std::{fs, usize};

const DIGIT_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn find_digit<T>(line: &str, indices_to_start_from: T) -> i32
    where T: Iterator<Item=(usize, char)>
{
    for (i, c) in indices_to_start_from {
        if let Some(d) = c.to_digit(10) {
            return d.try_into().unwrap();
        }

        let slice = &line[i..];

        for (d, digit_name) in DIGIT_NAMES.into_iter().enumerate() {
            if slice.starts_with(digit_name) {
                return d.try_into().unwrap();
            }
        }
    }

    panic!("Expected digit");
}

fn first_digit(line: &str) -> i32
{
    return find_digit(line, line.char_indices());
}

fn last_digit(line: &str) -> i32
{
    return find_digit(line, line.char_indices().rev());
}

fn main() {
    let path = r"src\input1.txt";
    let content = fs::read_to_string(path).expect("Read file");
    let sum: i32 = content.lines().map(|line| first_digit(line) * 10 + last_digit(line)).sum();
    println!("{}", sum);
}
use std::fs;

fn first_digit<T>(chars: T) -> i32 
    where T: Iterator<Item=char>
{
    return chars.into_iter()
        .find_map(|c| c.to_digit(10)).expect("At least one digit present")
        .try_into().expect("Digit in range");
}

fn main() {
    let path = r"src\input1.txt";
    let content = fs::read_to_string(path).expect("Read file");
    let sum: i32 = content.lines().map(|line| first_digit(line.chars()) * 10 + first_digit(line.chars().rev())).sum();
    println!("{}", sum);
}





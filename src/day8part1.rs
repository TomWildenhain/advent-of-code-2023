use std::fs;
use std::collections::HashMap;
use regex::Regex;


fn main() {
    let content = fs::read_to_string("./src/input8.txt").unwrap();
    let lines = content.lines();
    let mut lines_iter = lines.into_iter();
    let directions = lines_iter.next().unwrap();
    _ = lines_iter.next();  // Blank line

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let line_regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    for line in lines_iter {
        let (_, [src, dst1, dst2]) = line_regex.captures(line).unwrap().extract();
        network.insert(src, (dst1, dst2));
    }

    let mut loc: &str = "AAA";
    let mut steps = 0;
    let mut dir_i = 0;
    while loc != "ZZZ" {
        let direction = directions.as_bytes()[dir_i % directions.len()];
        let options = network.get(loc).unwrap();
        loc = match direction {
            b'L' => options.0,
            b'R' => options.1,
            _ => panic!("Invalid direction")
        };
        dir_i += 1;
        steps += 1;
    }

    println!("{}", steps);
}
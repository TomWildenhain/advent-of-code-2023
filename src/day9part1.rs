use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;

fn differences(history: &Vec<i32>) -> (Vec<i32>, bool) {
    let mut res = Vec::<i32>::new();
    let mut history_shifted = history.iter();
    history_shifted.next();
    let mut all_zero = true;
    for (a, b) in Iterator::zip(history.iter(), history_shifted) {
        if a != b {
            all_zero = false;
        }
        res.push(b - a);
    }

    return (res, all_zero);
}

fn next_value(history: Vec<i32>) -> i32 {
    let mut difference_stack: Vec<Vec<i32>> = vec![history];
    loop {
        let (next_entry, all_zero) = differences(difference_stack.last().unwrap());
        difference_stack.push(next_entry);
        if all_zero {
            break;
        }
    }

    let mut next_value: i32 = 0;
    for row in difference_stack.into_iter().rev() {
        next_value = row.last().unwrap() + next_value;
    }
    return next_value;
}


fn main() {
    let content = fs::read_to_string("./src/input9.txt").unwrap();
    let lines = content.lines();
    let histories = lines.map(
        |line| line.split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        );
    
    println!("{}", histories.map(next_value).sum::<i32>());

}
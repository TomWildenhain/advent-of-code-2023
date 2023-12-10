use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static!{
    static ref CARD_REGEX: Regex = Regex::new(r"Card *(\d*): ([\d ]*) \| ([\d ]*)").unwrap();
}

struct Card {
    scratched_numbers: Vec<i32>,
    winning_numbers: Vec<i32>
}

fn parse_card(line: &str) -> Card {
    let (_, [card_num, scratched_str, winning_str]) = CARD_REGEX.captures(line).unwrap().extract();

    return Card {
        scratched_numbers: scratched_str.split_whitespace().map(|d| str::parse(d).unwrap()).collect(),
        winning_numbers: winning_str.split_whitespace().map(|d| str::parse(d).unwrap()).collect(),
    }
}

fn part1() {
    let content = fs::read_to_string("./src/input4.txt").unwrap();
    let cards: Vec<Card> = content.lines().map(parse_card).collect();

    let winning_count: Vec<i32> = cards.iter().map(|_| 0).collect();

    let mut points = 0;
    for (i, card) in cards.iter().enumerate().rev() {
        let winning_count = card.scratched_numbers.iter().filter(|n| card.winning_numbers.contains(n)).count();
        let mut card_points = 0;
        for n in card.scratched_numbers.iter() {
            if card.winning_numbers.contains(n) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        points += card_points;
    }

    println!("{}", points);
}

fn part2() {
    let content = fs::read_to_string("./src/input4.txt").unwrap();
    let cards: Vec<Card> = content.lines().map(parse_card).collect();

    let mut winning_counts: Vec<i32> = cards.iter().map(|_| 0).collect();

    for (i, card) in cards.iter().enumerate().rev() {
        let count = card.scratched_numbers.iter().filter(|n| card.winning_numbers.contains(n)).count();
        winning_counts[i] = 1 + winning_counts[i+1..i+1+count].iter().sum::<i32>();
    }

    println!("{}", winning_counts.iter().sum::<i32>());
}

fn main() {
    part2();
}
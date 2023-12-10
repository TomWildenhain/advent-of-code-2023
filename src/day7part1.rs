use std::fs;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    bid: usize
}

const CARD_TYPES: &'static str = "23456789TJQKA";

const FIVE_OF_A_KIND: i32 = 7;
const FOUR_OF_A_KIND: i32 = 6;
const FULL_HOUSE: i32 = 5;
const THREE_OF_A_KIND: i32 = 4;
const TWO_PAIR: i32 = 3;
const ONE_PAIR: i32 = 2;
const HIGH_CARD: i32 = 1;

enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

fn parse_hand(line: &str) -> Hand {
    let (cards_str, bid_str) = line.split_once(' ').unwrap();

    return Hand {
        cards: cards_str.chars().map(|c| CARD_TYPES.find(c).unwrap()).collect(),
        bid: bid_str.parse().unwrap()
    }
}

fn get_hand_type(cards: &Vec<usize>) -> i32 {
    let mut card_to_counts: HashMap<usize, i32> = HashMap::new();
    for c in cards {
        let count = *card_to_counts.get(&c).unwrap_or(&0);
        card_to_counts.insert(*c, count + 1);
    }
    let mut counts: Vec<_> = card_to_counts.into_values().collect();
    counts.sort();
    counts.reverse();

    return match (counts[0], counts.get(1)) {
        (5, _) => FIVE_OF_A_KIND,
        (4, _) => FOUR_OF_A_KIND,
        (3, Some(2)) => FULL_HOUSE,
        (3, _) => THREE_OF_A_KIND,
        (2, Some(2)) => TWO_PAIR,
        (2, _) => ONE_PAIR,
        _ => HIGH_CARD
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HandResult {
    hand_type: i32,
    hand: Hand
}

impl Ord for HandResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        for (i, j) in Iterator::zip(self.hand.cards.iter(), other.hand.cards.iter()) {
            if i != j {
                return i.cmp(&j);
            }
        }

        return std::cmp::Ordering::Equal;
    }
}

impl PartialOrd for HandResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let content = fs::read_to_string("./src/input7.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();
    let hands = lines.into_iter().map(parse_hand);
    let mut hand_results: Vec<HandResult> = hands.map(|hand| HandResult {
        hand_type: get_hand_type(&hand.cards),
        hand: hand
    }).collect();

    hand_results.sort();


    println!("{}", hand_results.iter().enumerate().map(|(rank, res)| (rank+1) * res.hand.bid).sum::<usize>());
}
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

struct CubeCount {
    color: String,
    count: i32
}

struct CubeSet {
    counts: Vec<CubeCount>
}

struct Game {
    num: i32,
    handfuls: Vec<CubeSet>
}

fn parse_cube_count(cube_count_str: &str) -> CubeCount {
    let mut parts = cube_count_str.split(" ");
    let count = str::parse::<i32>(parts.next().unwrap()).unwrap();
    let color = parts.next().unwrap();
    return CubeCount { color: color.to_string(), count: count };
}

fn parse_handful(handful_str: &str) -> CubeSet {
    let cube_counts = handful_str.split(", ").map(|cube_count_str| parse_cube_count(cube_count_str));
    return CubeSet { counts: cube_counts.collect() };
}

lazy_static!{
    static ref GAME_REGEX: Regex = Regex::new(r"Game (\d*): (.*)").unwrap();
}

fn parse_game(line: &str) -> Game {
    let (_, [game_num_str, game_content]) = GAME_REGEX.captures(line).unwrap().extract();
    let handfuls = game_content.split("; ").map(|handful_str| parse_handful(handful_str));
    return Game { num: str::parse(game_num_str).unwrap(), handfuls: handfuls.collect()};
}

lazy_static!{
    static ref MAX_COUNTS: HashMap<&'static str, i32> = vec![
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ].into_iter().collect();
}

fn is_cube_count_possible(cube_count: &CubeCount) -> bool {
    if let Some(max_count) = MAX_COUNTS.get(cube_count.color.as_str()) {
        return cube_count.count <= *max_count;
    }

    return false;
}

fn is_handful_possible(handful: &CubeSet) -> bool {
    handful.counts.iter().all(is_cube_count_possible)
}

fn is_game_possible(game: &Game) -> bool {
    game.handfuls.iter().all(is_handful_possible)
}

fn part1() {
    let content = fs::read_to_string("./src/input2.txt").unwrap();
    let games = content.lines().map(|line| parse_game(line));

    let sum: i32 = games.filter(is_game_possible).map(|game| game.num).sum();

    println!("{}", sum);
}

fn get_game_power(game: &Game) -> i32 {
    let mut max_counts = HashMap::new();
    for handful in game.handfuls.iter() {
        for cube_count in handful.counts.iter() {
            let current_max = *max_counts.get(cube_count.color.as_str()).unwrap_or(&0);
            if cube_count.count > current_max {
                max_counts.insert(cube_count.color.as_str(), cube_count.count);
            }
        }
    }

    return max_counts.values().product();
}

fn part2() {
    let content = fs::read_to_string("./src/input2.txt").unwrap();
    let games = content.lines().map(|line| parse_game(line));

    let sum: i32 = games.map(|game| get_game_power(&game)).sum();

    println!("{}", sum);
}

fn main() {
    // part1();
    part2();
}
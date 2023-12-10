use std::fs;

fn parse_row(row: &str) -> Vec<i64> {
    row.split_once(":").unwrap().1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap()).collect()
}

fn ways_to_win(time: i64, best_dist: i64) -> i64 {
    let mut cnt = 0;
    for t in 0..time {
        let dist = t * (time - t);
        if dist > best_dist {
            cnt += 1;
        }
    }
    return cnt;
}

fn main() {
    let content = fs::read_to_string("./src/input6.txt").unwrap();
    let lines: Vec<_> = content.lines().collect();
    let times = parse_row(lines[0]);
    let dists = parse_row(lines[1]);

    let mut product = 1;

    for (time, dist) in Iterator::zip(times.into_iter(), dists.into_iter()) {
        product *= ways_to_win(time, dist);
    }

    println!("{}", product);
}
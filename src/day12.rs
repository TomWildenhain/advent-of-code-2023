use std::fs;
use std::collections::HashMap;

struct Row {
    record: String,
    groups: Vec<usize>
}

fn parse_row5(line: &str) -> Row {
    let row = parse_row(line);

    let new_record = (row.record + "?").repeat(5);

    return Row {
        record: new_record[..new_record.len() - 1].to_string(),
        groups: row.groups.repeat(5)
    }
}

fn parse_row(line: &str) -> Row {
    let (record_part, group_part) = line.split_once(' ').unwrap();
    let groups = group_part.split(',').map(|s| s.parse().unwrap()).collect();
    return Row {
        record: record_part.to_string(),
        groups: groups
    }
}

fn count_combinations_memo(row: &Row, record_i: usize, groups_i: usize, map: &mut HashMap<(usize, usize), i64>) -> i64 {
    let key = (record_i, groups_i);
    if let Some(res) = map.get(&key) {
        return *res;
    }

    let answer = count_combinations_impl(row, record_i, groups_i, map);

    map.insert(key, answer);

    return answer;
}

fn count_combinations_impl(row: &Row, record_i: usize, groups_i: usize, map: &mut HashMap<(usize, usize), i64>) -> i64 {
    // println!("{}, {}", record_i, groups_i);

    let group = row.groups[groups_i];

    if record_i + group > row.record.len() {
        return 0;
    }

    let record_part: &str = &row.record[record_i..record_i + group];
    if record_part.chars().any(|c| c == '.') {
        return 0;
    }

    if record_i + group == row.record.len() {
        if groups_i + 1 == row.groups.len() {
            // println!("Add!");
            return 1;
        }
        return 0;
    }

    if row.record.as_bytes()[record_i + group] == b'#' {
        return 0;
    }

    if groups_i + 1 == row.groups.len() {
        let new_record_i = record_i + group + 1;
        if row.record[new_record_i..].chars().all(|c| c != '#') {
            // println!("Add 2!");
            return 1;
        }
        return 0;
    }

    if record_i + group + 1 == row.record.len() {
        return 0;
    }

    let mut sum = 0;
    for i in record_i + group + 1..row.record.len() {
        if row.record.as_bytes()[i - 1] == b'#' {
            break;
        }
        sum += count_combinations_memo(row, i, groups_i + 1, map);
    }

    return sum;
}

fn count_combinations(row: &Row) -> i64 {
    let mut map = HashMap::new();

    let mut sum = 0;
    for i in 0..row.record.len() {
        if i > 0 && row.record.as_bytes()[i - 1] == b'#' {
            break;
        }
        sum += count_combinations_memo(row, i, 0, &mut map);
    }
    return sum;
}

fn main() {
    let content = fs::read_to_string("./src/input12.txt").unwrap();
    let rows = content.lines().map(parse_row5);

    let mut sum = 0;
    for row in rows {
        let combinations = count_combinations(&row);
        sum += combinations;
        println!("Combinations: {}", combinations);
    }

    println!("Sum: {}", sum);
}

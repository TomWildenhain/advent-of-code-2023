use std::fs;

struct Stone {
    position: Vec<i128>,
    velocity: Vec<i128>
}

const MIN_INTERSECTION: i128 = 200000000000000;
const MAX_INTERSECTION: i128 = 400000000000000;

// const MIN_INTERSECTION: i128 = 7;
// const MAX_INTERSECTION: i128 = 27;

fn intersect_in_range(stone1: &Stone, stone2: &Stone) -> bool {
    let t1_term1 = (stone2.position[0] - stone1.position[0]) * stone2.velocity[1];
    let t1_term2 = (stone1.position[1] - stone2.position[1]) * stone2.velocity[0];
    let t1_num = t1_term1 + t1_term2;

    let t2_term1 = -(stone1.position[0] - stone2.position[0]) * stone1.velocity[1];
    let t2_term2 = -(stone2.position[1] - stone1.position[1]) * stone1.velocity[0];
    let t2_num = t2_term1 + t2_term2;
    
    let d_term1 = stone1.velocity[0] * stone2.velocity[1];
    let d_term2 = -stone1.velocity[1] * stone2.velocity[0];
    
    let denom = d_term1 + d_term2;
    
    let num_x1 = t1_num * stone1.velocity[0] + stone1.position[0] * denom;
    let num_x2 = t2_num * stone2.velocity[0] + stone2.position[0] * denom;
    let num_y1 = t1_num * stone1.velocity[1] + stone1.position[1] * denom;
    let num_y2 = t2_num * stone2.velocity[1] + stone2.position[1] * denom;

    if denom == 0 {
        return false;
    }

    if t1_num * denom < 0 || t2_num * denom < 0 {
        return false;
    }

    assert_eq!(num_x1, num_x2);
    assert_eq!(num_y1, num_y2);

    if !(MIN_INTERSECTION * denom <= num_x1 && num_x1 <= MAX_INTERSECTION * denom) {
        return false;
    }

    if !(MIN_INTERSECTION * denom <= num_y1 && num_y1 <= MAX_INTERSECTION * denom) {
        return false;
    }

    return true;
}

fn parse_tuple(tuple_str: &str) -> Vec<i128> {
    let parts: Result<Vec<i128>, _> = tuple_str.split(", ").map(|s| s.parse()).collect();
    return parts.unwrap()
}

fn parse_stone(line: &str) -> Stone {
    let (positions_str, velocity_str) = line.split_once(" @ ").unwrap();
    return Stone {
        position: parse_tuple(positions_str),
        velocity: parse_tuple(velocity_str),
    }
}

fn main() {
    let content = fs::read_to_string("./src/input24.txt").unwrap();
    let stones: Vec<_> = content.lines().map(parse_stone).collect();

    let mut cnt = 0;
    for s1 in 0..stones.len() {
        for s2 in 0..stones.len() {
            if s1 == s2 {
                continue;
            }
            if intersect_in_range(&stones[s1], &stones[s2]) {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
use std::fs;

type Direction = (i64, i64);

struct Instruction {
    direction: Direction,
    distance: i64,
}

fn match_direction(direction: &str) -> Direction {
    return match direction {
        "U" => (0, -1),
        "D" => (0, 1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!()
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split(' ');

    return Instruction {
        direction: match_direction(parts.next().unwrap()),
        distance: parts.next().unwrap().parse().unwrap(),
    }
}

fn parse_instruction_part2(line: &str) -> Instruction {
    let raw_hex = line.split(' ').nth(2).unwrap();
    let hex = &raw_hex[2..raw_hex.len()-1];

    let dist = i64::from_str_radix(&hex[..5], 16).unwrap();
    let dir = match &hex[5..] {
        "0" => (1, 0),
        "1" => (0, 1),
        "2" => (-1, 0),
        "3" => (0, -1),
        _ => panic!()
    };

    return Instruction {
        direction: dir,
        distance: dist
    };
}

fn compute_area(instructions: Vec<Instruction>) -> i64 {
    let mut total_len = 0;
    let mut curr_y = 0;
    let mut area = 0;

    for instr in instructions {
        let (dx, dy) = instr.direction;
        curr_y += dy * instr.distance;
        area += curr_y * dx * instr.distance;
        total_len += instr.distance;
    }

    let adjusted_area = total_len / 2 + area.abs() + 1;

    return adjusted_area;
}

fn main() {
    let contents = fs::read_to_string("./src/input18.txt").unwrap();
    // let instructions: Vec<Instruction> = contents.lines().map(parse_instruction).collect();
    let instructions: Vec<Instruction> = contents.lines().map(parse_instruction_part2).collect();
    println!("{}", compute_area(instructions));
}
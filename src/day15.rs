use std::fs;

fn compute_hash(s: &str) -> usize {
    let mut current: usize = 0;
    for b in s.bytes() {
        let b_usize: usize = b.into();
        current = ((current + b_usize) * 17) % 256;
    }

    return current;
}

fn part1() {
    let content = fs::read_to_string("./src/input15.txt").unwrap();
    let parts: Vec<_> = content.trim().split(',').collect();

    println!("Sum: {}", parts.into_iter().map(compute_hash).sum::<usize>());
}

type Boxes<'a> = Vec<Vec<(&'a str, usize)>>;

// Look, I'm using lifetimes!
fn assign<'a, 'b>(boxes: &mut Boxes<'a>, label: &'b str, value: usize)
    where 'b: 'a {
    let hash = compute_hash(label);

    if let Some(idx) = boxes[hash].iter().position(|x| x.0 == label) {
        boxes[hash][idx] = (label, value);
    } else {
        boxes[hash].push((label, value));
    }
}

fn delete(boxes: &mut Boxes, label: &str) {
    let hash = compute_hash(label);

    if let Some(idx) = boxes[hash].iter().position(|x| x.0 == label) {
        boxes[hash].remove(idx);
    }
}

fn focusing_power(boxes: &Boxes) -> usize {
    let mut sum = 0;

    for (i, b) in boxes.iter().enumerate() {
        for (j, entry) in b.iter().enumerate() {
            sum += (i + 1) * (j + 1) * entry.1;
        }
    }

    return sum;
}

fn part2() {
    let content = fs::read_to_string("./src/input15.txt").unwrap();
    let instructions: Vec<_> = content.trim().split(',').collect();

    let mut boxes: Boxes = (0..256).map(|_| vec![]).collect();

    for instruction in instructions {
        if instruction.ends_with('-') {
            let label = &instruction[..instruction.len() - 1];
            delete(&mut boxes, label);
        }
        else {
            let (label, value) = instruction.split_once('=').unwrap();
            assign(&mut boxes, label, value.parse().unwrap());
        }
    }

    println!("{}", focusing_power(&boxes));
}

fn main() {
    part2();
}
use std::fs;

type Grid<T> = Vec<Vec<T>>;

struct Brick {
    start: Vec<usize>,
    end: Vec<usize>
}

impl Brick {
    fn xy_coords(&self) -> impl Iterator<Item=(usize, usize)> + '_ {
        (self.start[1]..=self.end[1]).flat_map(move |y| (self.start[0]..=self.end[0]).map(move |x| (x, y)))
    }

    fn height(&self) -> usize {
        return self.end[2] - self.start[2] + 1;
    }
}

fn parse_tuple(tuple_str: &str) -> Option<Vec<usize>> {
    let parts: Result<Vec<usize>, _> = tuple_str.split(",").map(|s| s.parse()).collect();
    if let Ok(some_parts) = parts {
        return Some(some_parts);
    }

    return None;
}

fn parse_brick(line: &str) -> Brick {
    let (start_str, end_str) = line.split_once("~").unwrap();

    return Brick {
        start: parse_tuple(start_str).unwrap(),
        end: parse_tuple(end_str).unwrap()
    }
}

#[derive(Clone, Debug)]
struct BrickGridEntry {
    z: usize,
    brick_i: usize
}

fn part1(bricks_above_brick: &Vec<Vec<usize>>, bricks_below_brick: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for i in 0..bricks_above_brick.len() {
        let can_remove = bricks_above_brick[i].iter().all(|brick_above| {
            return bricks_below_brick[*brick_above].len() > 1;
        });

        if can_remove {
            count += 1;
        }
    }
    return count;
}

fn chain_disintegrate(bricks_above_brick: &Vec<Vec<usize>>, bricks_below_brick: &Vec<Vec<usize>>, on_ground: &Vec<bool>, brick: usize) -> usize {
    let n = bricks_above_brick.len();
    let mut disintigrated = vec![false; n];
    disintigrated[brick] = true;
    let mut cnt = 0;

    let mut progress = true;
    while progress {
        progress = false;
        for i in 0..n {
            if !disintigrated[i] && !on_ground[i] && bricks_below_brick[i].iter().all(|b| disintigrated[*b]) {
                disintigrated[i] = true;
                progress = true;
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn part2(bricks_above_brick: &Vec<Vec<usize>>, bricks_below_brick: &Vec<Vec<usize>>, on_ground: &Vec<bool>) -> usize {
    let mut sum = 0;
    for i in 0..bricks_above_brick.len() {
        sum += chain_disintegrate(bricks_above_brick, bricks_below_brick, on_ground, i);
    }
    return sum;
}

fn main() {
    let content = fs::read_to_string("./src/input22.txt").unwrap();

    let mut bricks: Vec<_> = content.lines().map(parse_brick).collect();
    bricks.sort_by_key(|b| b.start[2]);

    let ground_idx = bricks.len() + 1;

    let width = bricks.iter().map(|b| b.start[0]).max().unwrap() + 1;
    let height = bricks.iter().map(|b| b.start[1]).max().unwrap() + 1;

    let mut bricks_above_brick: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut bricks_below_brick: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut on_ground: Vec<bool> = vec![false; bricks.len()];

    let mut grid: Grid<BrickGridEntry> = vec![vec![BrickGridEntry {z: 0, brick_i: ground_idx}; width]; height];

    for (i, brick) in bricks.iter().enumerate() {
        let max_z = brick.xy_coords().map(|(x, y)| grid[y][x].z).max().unwrap();
        for (x, y) in brick.xy_coords() {
            let entry = &grid[y][x];
            if entry.z == max_z {
                if entry.brick_i == ground_idx {
                    on_ground[i] = true;
                }
                if entry.brick_i != ground_idx && !bricks_below_brick[i].contains(&entry.brick_i) {
                    bricks_below_brick[i].push(entry.brick_i);
                    bricks_above_brick[entry.brick_i].push(i);
                }
            }

            grid[y][x] = BrickGridEntry {z: max_z + brick.height(), brick_i: i};
        }
    }

    println!("{}", part1(&bricks_above_brick, &bricks_below_brick));
    println!("{}", part2(&bricks_above_brick, &bricks_below_brick, &on_ground));
}
use std::fs;

fn expanded_distance(start: usize, end: usize, expanded: &Vec<usize>) -> usize {
    let low = start.min(end);
    let high = start.max(end);
    let mut dist = high - low;
    for x in expanded {
        if low < *x && *x < high {
            dist += 1000000 - 1;  // Set to 1 for part 1
        }
    }
    return dist;
}

fn main() {
    let content = fs::read_to_string("./src/input11.txt").unwrap();
    let grid: Vec<_> = content.lines().map(|l| l.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut sum = 0;

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for r in 0..height {
        if (0..width).all(|c| grid[r][c] == b'.') {
            empty_rows.push(r);
        }
    }

    for c in 0..width {
        if (0..height).all(|r| grid[r][c] == b'.') {
            empty_cols.push(c);
        }
    }

    for r1 in 0..height {
        for c1 in 0..width {
            let byte1 = grid[r1][c1];
            if byte1 != b'#' {
                continue;
            }
            for r2 in 0..height {
                for c2 in 0..width {
                    let byte2 = grid[r2][c2];
                    if byte2 != b'#' {
                        continue;
                    }
                    if r1 == r2 && c1 == c2 {
                        continue;
                    }
                    if r2 < r1 || (r2 == r1 && c2 < c1) {
                        // Include each pair exactly once
                        continue;
                    }
                    sum += expanded_distance(r1, r2, &empty_rows) + 
                        expanded_distance(c1, c2, &empty_cols)
                }
            }
        }
    }

    println!("Sum: {}", sum);
}
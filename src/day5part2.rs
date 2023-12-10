use std::fs;
use std::cmp;
use regex::Regex;

#[derive(Debug)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn end(&self) -> i64 { self.start + self.length }
}

fn range_intersect(r1: &Range, r2: &Range) -> Option<Range> {
    let start = cmp::max(r1.start, r2.start);
    let end = cmp::min(r1.end(), r2.end());
    if start < end {
        return Some(Range {
            start: start,
            length: end - start
        })
    }

    return None;
}

struct MapEntry {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

impl MapEntry {
    fn start_range(&self) -> Range {
        return Range {
            start: self.src_start,
            length: self.range_len
        }
    }

    fn contains_start(&self, value: i64) -> bool {
        return self.src_start <= value && value < self.src_start + self.range_len;
    }

    fn shift(&self) -> i64 {
        return self.dst_start - self.src_start;
    }
}

struct AlmanacMap {
    src_type: String,
    dst_type: String,
    entries: Vec<MapEntry>
}

impl AlmanacMap {
    fn lookup_range(&self, range: &Range) -> Vec<Range> {
        let index = self.entries.binary_search_by_key(&range.start, |entry| entry.src_start);
        let closest_index = match index {
            Ok(i) => i,
            Err(i) => if i == 0 {0} else {i - 1}
        };

        let mut current_index = closest_index;
        let mut ranges = Vec::<Range>::new();

        let mut remaining_start = range.start;
        let range_end = range.end();

        while remaining_start < range_end {
            if current_index >= self.entries.len() {
                let new_range = Range {start: remaining_start, length: range_end - remaining_start};
                remaining_start = new_range.end();
                ranges.push(new_range);
                continue;
            }
            if self.entries[current_index].start_range().end() <= remaining_start {
                current_index += 1;
                continue;
            }
            if remaining_start < self.entries[current_index].src_start {
                let new_range = Range {start: remaining_start, length: self.entries[current_index].src_start - remaining_start};
                remaining_start = new_range.end();
                ranges.push(new_range);
                continue;
            }
            if current_index < self.entries.len() && self.entries[current_index].contains_start(remaining_start) {
                if let Some(new_start_range) = range_intersect(&self.entries[current_index].start_range(), &Range {start: remaining_start, length: range_end - remaining_start}) {
                    remaining_start = new_start_range.end();
                    let new_end_range = Range {start: new_start_range.start + self.entries[current_index].shift(), length: new_start_range.length};
                    ranges.push(new_end_range);
                    continue;
                }
            }
        }

        return ranges;
    }

    fn lookup(&self, src: i64) -> i64 {
        let index = self.entries.binary_search_by_key(&src, |entry| entry.src_start);
        let closest_index = match index {
            Ok(i) => i,
            Err(i) => if i == 0 {0} else {i - 1}
        };
        
        if closest_index < self.entries.len() {
            let entry = &self.entries[closest_index];
            if src >= entry.src_start && src < entry.src_start + entry.range_len {
                return entry.dst_start + (src - entry.src_start);
            }
        }

        return src;
    }
}

fn find_min_location(range: Range, maps: &[&AlmanacMap]) -> i64 {
    if let Some(map) = maps.first() {
        let mut min = i64::MAX;
        // println!("Range: {:?}", range);
        let ranges = map.lookup_range(&range);
        // println!("Ranges: {:?}", ranges);
        // panic!("Done");
        for next_range in ranges {
            min = cmp::min(min, find_min_location(next_range, &maps[1..]))
        }
        return min;
    }
    return range.start;
}

fn main() {
    let content = fs::read_to_string("./src/input5.txt").unwrap();
    let lines = content.lines();
    let mut lines_iter = lines.into_iter();

    let seeds_line = lines_iter.next().unwrap();

    let seeds: Vec<i64> = seeds_line
        .split_once(": ").unwrap().1
        .split(" ")
        .map(|seed_str| str::parse::<i64>(seed_str).unwrap())
        .collect();

    let mut seed_ranges: Vec<Range> = vec![];
    let mut seed_iter = seeds.iter();
    loop {
        if let Some(d) = seed_iter.next() {
            seed_ranges.push(Range {start: *d, length: *seed_iter.next().unwrap()})
        } else {
            break;
        }
    }

    let map_type_regex = Regex::new("(.*)-to-(.*) map:").unwrap();

    let mut maps = Vec::<AlmanacMap>::new();

    for line in lines_iter {
        if line.ends_with(" map:") {
            let (_, [src_type, dst_type]) = map_type_regex.captures(line).unwrap().extract();
            maps.push(AlmanacMap {
                src_type: src_type.to_string(), 
                dst_type: dst_type.to_string(), 
                entries: Vec::new()
            });
        }
        else if line.len() > 0 {
            let current_map = maps.last_mut().unwrap();
            let parts: Vec<&str> = line.split(' ').collect();
            current_map.entries.push(MapEntry {
                dst_start: str::parse(parts[0]).unwrap(),
                src_start: str::parse(parts[1]).unwrap(),
                range_len: str::parse(parts[2]).unwrap(),
            });
        }
    }

    for map in maps.iter_mut() {
        map.entries.sort_by_key(|entry| entry.src_start);
    }

    let map_refs: Vec<_> = maps.iter().collect();

    let mut min = i64::MAX;
    for next_range in seed_ranges {
        min = cmp::min(min, find_min_location(next_range, &map_refs[..]))
    }

    println!("{}", min);
}
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct MapEntry {
    src_start: i64,
    dst_start: i64,
    range_len: i64,
}

#[derive(Debug)]
struct AlmanacMap {
    src_type: String,
    dst_type: String,
    entries: Vec<MapEntry>
}

impl AlmanacMap {
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

    let find_location = |seed: i64| {
        let mut val = seed;
        for map in maps.iter() {
            val = map.lookup(val);
        }
        return val;
    };

    println!("{}", seeds.into_iter().map(find_location).min().unwrap());
}
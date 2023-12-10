use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;


fn main() {
    let content = fs::read_to_string("./src/input8.txt").unwrap();
    let lines = content.lines();
    let mut lines_iter = lines.into_iter();
    let directions = lines_iter.next().unwrap();
    _ = lines_iter.next();  // Blank line

    let mut network: HashMap<&str, (&str, &str)> = HashMap::new();
    let line_regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
    for line in lines_iter {
        let (_, [src, dst1, dst2]) = line_regex.captures(line).unwrap().extract();
        network.insert(src, (dst1, dst2));
    }

    let mut steps: usize = 0;
    let mut nodes: Vec<&str> = Vec::<&str>::from_iter(network.keys().filter(|node| node.ends_with("A")).map(|node| *node));
    println!("Nodes: {}", nodes.len());
    let mut has_non_z = true;
    let mut nodes_to_z: Vec<Vec<usize>> = nodes.iter().map(|_| vec![]).collect();
    while has_non_z {
        let mut new_nodes: Vec<&str> = Vec::new();
        has_non_z = false;
        let direction_idx = steps % directions.len();
        let direction = directions.as_bytes()[direction_idx];

        for (i, node) in nodes.iter().enumerate() {
            let options = network.get(node).unwrap();
            let new_node = match direction {
                b'L' => options.0,
                b'R' => options.1,
                _ => panic!("Invalid direction")
            };
            new_nodes.push(new_node);
            if !new_node.ends_with("Z") {
                has_non_z = true;
            } else if nodes_to_z[i].len() < 4 {
                nodes_to_z[i].push(steps + 1);
            }
        }
        
        nodes = new_nodes;
        if steps % 50000 == 0 {
            println!("Nodes: {}", nodes.len());
            println!("Nodes: {:?}", nodes_to_z);
            println!("{}", steps);
        }
        steps += 1;
    }

    println!("{}", steps);
}
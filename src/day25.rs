use std::fs;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::{thread_rng,Rng};
use std::mem;

struct UnionFind {
    sets: Vec<Vec<usize>>,
    elt_to_set: Vec<usize>
}

impl UnionFind {
    // A simple but fast enough implementation of union find. 
    // The secret sauce is always copying from the smaller set to the larger set.
    fn new(n: usize) -> UnionFind {
        UnionFind {
            sets: (0..n).map(|n| vec![n]).collect(),
            elt_to_set: (0..n).collect()
        }
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let set1 = self.elt_to_set[a];
        let set2 = self.elt_to_set[b];
        
        if set1 == set2 {
            return false;
        }

        if self.sets[set1].len() < self.sets[set2].len() {
            // Move set1 to set2
            let mut set1_contents = vec![];
            mem::swap(&mut set1_contents, &mut self.sets[set1]);
            self.sets[set2].extend_from_slice(&set1_contents);
            for x in set1_contents {
                self.elt_to_set[x] = set2;
            }
        }
        else {
            // Move set2 to set1
            let mut set2_contents = vec![];
            mem::swap(&mut set2_contents, &mut self.sets[set2]);
            self.sets[set1].extend_from_slice(&set2_contents);
            for x in set2_contents {
                self.elt_to_set[x] = set1;
            }
        }

        return true;
    }

    fn find(&self, a: usize) -> usize {
        return self.elt_to_set[a];
    }
}

fn randomized_min_cut<R: Rng>(edges: &Vec<(usize, usize)>, n: usize, rng: &mut R) -> (Vec<(usize, usize)>, (usize, usize)) {
    // Thanks 15-251!

    let mut edges = edges.clone();
    edges.shuffle(rng);

    let mut union_find = UnionFind::new(n);
    let mut num_components = n;
    let mut edge_idx = 0;

    while num_components > 2 {
        let (a, b) = edges[edge_idx];
        if union_find.union(a, b) {
            num_components -= 1;
        }

        edge_idx += 1;
    }

    let mut cut_edges: Vec<(usize, usize)> = vec![];
    let mut component_sizes = (0, 0);

    for (a, b) in &edges[edge_idx..] {
        let set1 = union_find.find(*a);
        let set2 = union_find.find(*b);
        if set1 != set2 {
            cut_edges.push((*a, *b));
            component_sizes = (union_find.sets[set1].len(), union_find.sets[set2].len())
        }
    }

    return (cut_edges, component_sizes);
}

fn get_or_assign_idx<'a>(vertex_to_idx: &mut HashMap<&'a str, usize>, vertex: &'a str) -> usize {
    let next_idx = vertex_to_idx.len();
    let entry = vertex_to_idx.entry(vertex);
    return *entry.or_insert(next_idx);
}

fn main() {
    let content = fs::read_to_string("./src/input25.txt").unwrap();
    
    let mut rng = thread_rng();
    let mut vertex_to_idx: HashMap<&str, usize> = HashMap::new();
    let mut edges: Vec<(usize, usize)> = vec![];

    for line in content.lines() {
        let (src, dst) = line.split_once(": ").unwrap();
        let src_idx = get_or_assign_idx(&mut vertex_to_idx, src);

        let dsts = dst.split(' ');
        for dst in dsts {
            let dst_idx = get_or_assign_idx(&mut vertex_to_idx, dst);
            edges.push((src_idx, dst_idx));
        }
    }

    println!("n = {}, m = {}", vertex_to_idx.len(), edges.len());

    loop {
        let (cut_edges, component_sizes) = randomized_min_cut(&edges, vertex_to_idx.len(), &mut rng);
        if cut_edges.len() == 3 {
            let product = component_sizes.0 * component_sizes.1;

            println!("cut_edges = {:?}, component_sizes = {:?}, product = {}", cut_edges, component_sizes, product);
            break;
        }
    }
}

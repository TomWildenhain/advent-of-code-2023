use std::fs;

type Grid<T> = Vec<Vec<T>>;

fn blank_grid<T: Copy>(width: usize, height: usize, item: T) -> Grid<T> {
    return (0..height).map(|_| (0..width).map(|_| item).collect()).collect();
}

fn blank_grid_fn<T, F: FnMut((usize, usize)) -> T>(width: usize, height: usize, mut function: F) -> Grid<T> {
    return (0..height).map(|y| (0..width).map(|x| function((x, y))).collect()).collect();
}

type Direction = (isize, isize);

const DIRECTIONS: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn step_in_direction(point: (usize, usize), direction: Direction, width: usize, height: usize) -> Option<(usize, usize)> {
    let (x, y) = point;
    let (dx, dy) = direction;

    let new_x = x.checked_add_signed(dx)?;
    let new_y = y.checked_add_signed(dy)?;
    if new_x < width && new_y < height {
        return Some((new_x, new_y));
    }

    return None;
}

fn neighbors(point: (usize, usize), width: usize, height: usize) -> impl Iterator<Item=(usize, usize)> {
    return DIRECTIONS.iter().filter_map(move |d| step_in_direction(point, *d, width, height));
}

fn row_to_string(row: &Vec<bool>) -> String {
    let strs: Vec<_> = row.iter().map(|c| if *c {"#"} else {"."}).collect();
    return strs.join("");
}

fn grid_to_string(grid: &Grid<bool>) -> String {
    let strs: Vec<_> = grid.iter().map(row_to_string).collect();
    return strs.join("\n");
}

// Turns out, I don't need to do it this way. Oh well.
#[derive(Debug)]
struct Edge {
    weight: usize,
}

#[derive(Debug)]
struct HalfEdge {
    edge: usize,
    other_v: usize,
}

#[derive(Debug)]
struct Graph {
    start_v: usize,
    end_v: usize,
    edges: Vec<Edge>,
    v_to_edges: Vec<Vec<HalfEdge>>,
}

// __          __     _____  _   _ _____ _   _  _____ 
// \ \        / /\   |  __ \| \ | |_   _| \ | |/ ____|
//  \ \  /\  / /  \  | |__) |  \| | | | |  \| | |  __ 
//   \ \/  \/ / /\ \ |  _  /| . ` | | | | . ` | | |_ |
//    \  /\  / ____ \| | \ \| |\  |_| |_| |\  | |__| |
//     \/  \/_/    \_\_|  \_\_| \_|_____|_| \_|\_____|
//
// The following Rust code is terrible. It is bad in every way. Shockingly, it works.

struct LkGrHalfEdge {
    v: usize,
    twin: usize,
    weight: usize,
    translated_e: Option<usize>
}

struct LkGrVertex {
    half_edges: Vec<usize>,
    translated_v: usize
}

fn compress_lk_graph(mut vertices: Vec<Option<LkGrVertex>>, mut edges: Vec<Option<LkGrHalfEdge>>, start_v: usize, end_v: usize) -> Graph {
    // I praise the programming gods that this code worked and I did not have to debug this.
    let mut progess = true;
    println!("Compressing!");
    while progess {
        progess = false;
        for v_i in 0..vertices.len() {
            if vertices[v_i].is_some() && vertices[v_i].as_ref().unwrap().half_edges.len() == 2 {
                // I took care to address all compiler errors elegantly. Jk, I copy-pasted ".as_ref()" everywhere.
                let [e1, e2] = vertices[v_i].as_ref().unwrap().half_edges[..] else {panic!()};
                let w = edges[e1].as_ref().unwrap().weight + edges[e2].as_ref().unwrap().weight;
                let v1 = edges[e1].as_ref().unwrap().v;
                let v2 = edges[e2].as_ref().unwrap().v;
                let t1 = edges[e1].as_ref().unwrap().twin;
                let t2 = edges[e2].as_ref().unwrap().twin;

                let t1_edge = edges[t1].as_mut().unwrap();
                t1_edge.v = v2;
                t1_edge.weight = w;
                t1_edge.twin = t2;

                let t2_edge = edges[t2].as_mut().unwrap();
                t2_edge.v = v1;
                t2_edge.weight = w;
                t2_edge.twin = t1;

                edges[e1] = None;
                edges[e2] = None;
                vertices[v_i] = None;
                progess = true;
            }
        }
    }

    let mut new_v_cnt = 0;
    for v_i in 0..vertices.len() {
        if vertices[v_i].is_some() {
            vertices[v_i].as_mut().unwrap().translated_v = new_v_cnt;
            new_v_cnt += 1;
        }
    }

    let mut new_edges: Vec<Edge> = vec![];
    let mut new_edge_cnt = 0;
    for e_i in 0..edges.len() {
        if edges[e_i].is_some() && edges[e_i].as_ref().unwrap().translated_e.is_none() {
            new_edges.push(Edge {weight: edges[e_i].as_ref().unwrap().weight});
            edges[e_i].as_mut().unwrap().translated_e = Some(new_edge_cnt);
            let t = edges[e_i].as_ref().unwrap().twin;
            edges[t].as_mut().unwrap().translated_e = Some(new_edge_cnt);
            new_edge_cnt += 1;
        }
    }

    let mut new_v_to_edges: Vec<Vec<HalfEdge>> = (0..new_v_cnt).map(|_| vec![]).collect();
    for v_i in 0..vertices.len() {
        if vertices[v_i].is_some() {
            for e in vertices[v_i].as_ref().unwrap().half_edges.iter() {
                let v2 = edges[*e].as_ref().unwrap().v;
                new_v_to_edges[vertices[v_i].as_ref().unwrap().translated_v].push(
                    HalfEdge {
                        edge: edges[*e].as_ref().unwrap().translated_e.unwrap(), 
                        other_v: vertices[v2].as_ref().unwrap().translated_v
                    });
            }
        }
    }

    println!("Done!");

    return Graph {
        start_v: vertices[start_v].as_ref().unwrap().translated_v,
        end_v: vertices[end_v].as_ref().unwrap().translated_v,
        edges: new_edges,
        v_to_edges: new_v_to_edges
    }
}

fn make_graph(grid: &Grid<u8>, start: (usize, usize), end: (usize, usize)) -> Graph {
    let height = grid.len();
    let width = grid[0].len();

    let (start_x, start_y) = start;
    let (end_x, end_y) = end;

    let mut vertices: Vec<Option<LkGrVertex>> = vec![];
    let vertices_grid: Grid<usize> = blank_grid_fn(width, height, |(x, y)| {
        if grid[y][x] != b'#' {
            let idx = vertices.len();
            vertices.push(Some(LkGrVertex { half_edges: vec![], translated_v: 0 }));
            idx
        } else {
            0
        }
    });

    let mut edges: Vec<Option<LkGrHalfEdge>> = vec![];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != b'#' {
                let v_i = vertices_grid[y][x];
                for (x2, y2) in neighbors((x, y), width, height) {
                    if grid[y2][x2] != b'#' {
                        if x < x2 || (x == x2 && y < y2) {
                            let v2_i = vertices_grid[y2][x2];
                            let e_i = edges.len();
                            let t_i = edges.len() + 1;
                            edges.push(Some(LkGrHalfEdge {v: v2_i, twin: t_i, weight: 1, translated_e: None}));
                            edges.push(Some(LkGrHalfEdge {v: v_i, twin: e_i, weight: 1, translated_e: None}));
                            vertices[v_i].as_mut().unwrap().half_edges.push(e_i);
                            vertices[v2_i].as_mut().unwrap().half_edges.push(t_i);
                        }
                    }
                }
            }
        }
    }

    let graph = compress_lk_graph(vertices, edges, vertices_grid[start_y][start_x], vertices_grid[end_y][end_x]);

    let degrees: Vec<_> = graph.v_to_edges.iter().map(|v| v.len()).collect();
    println!("Degrees: {:?}", degrees);
    println!("Edges: {}, Vertices: {}", graph.edges.len(), graph.v_to_edges.len());

    return graph;
}

// A hacky little container I added for debugging.
struct Best {
    best_so_far: usize
}

// Once we make the graph the code isn't so bad.
fn longest_hike(graph: &Graph, visited_vertices: &mut Vec<bool>, current_length: usize, current_v: usize, best_so_far: &mut Best) -> usize {
    if current_v == graph.end_v {
        if current_length > best_so_far.best_so_far {
            println!("{}", current_length);
            best_so_far.best_so_far = current_length;
        }
        return current_length;
    }

    let mut max_hike = 0;
    for half_edge in graph.v_to_edges[current_v].iter() {
        if !visited_vertices[half_edge.other_v] {
            let weight = graph.edges[half_edge.edge].weight;
            visited_vertices[half_edge.other_v] = true;
            max_hike = max_hike.max(longest_hike(graph, visited_vertices, current_length + weight, half_edge.other_v, best_so_far));
            visited_vertices[half_edge.other_v] = false;
        }
    }

    return max_hike;
}

fn part2() {
    let content = fs::read_to_string("./src/input23.txt").unwrap();
    let grid: Grid<u8> = content.lines().map(|l| l.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let start_x = content.lines().next().unwrap().find(".").unwrap();
    let start_y = 0;

    let end_x = content.lines().last().unwrap().find(".").unwrap();
    let end_y = height - 1;

    let graph = make_graph(&grid, (start_x, start_y), (end_x, end_y));
    println!("{:?}", graph);

    let mut visited_vertices = vec![false; graph.v_to_edges.len()];
    visited_vertices[graph.start_v] = true;
    let mut best = Best { best_so_far: 0 };

    let longest = longest_hike(&graph, &mut visited_vertices, 0, graph.start_v, &mut best);
    println!("{}", longest);
}

use std::thread;

const STACK_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size (https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/)
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(part2)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
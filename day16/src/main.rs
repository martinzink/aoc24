use self::Direction::*;
use petgraph::data::DataMap;
use petgraph::dot::{Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::{EdgeRef, IntoEdgesDirected, NodeRef};
use petgraph::{Graph};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, Write};
use std::process::Command;
use std::slice::Iter;
use utils::coord::Coord;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, West, South, East];
        DIRECTIONS.iter()
    }

    pub fn get_left_right_neigbhours(&self) -> Vec<Direction> {
        match self {
            North => {
                vec![East, West]
            }
            West => {
                vec![North, South]
            }
            South => {
                vec![East, West]
            }
            East => {
                vec![North, South]
            }
        }
    }

    pub fn get_vector(&self) -> utils::coord::Coord {
        match self {
            North => utils::coord::Coord { x: 0, y: -1 },
            West => utils::coord::Coord { x: -1, y: 0 },
            South => utils::coord::Coord { x: 0, y: 1 },
            East => utils::coord::Coord { x: 1, y: 0 },
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match self {
            North => South,
            West => East,
            South => North,
            East => West,
        }
    }
}

fn export_to_png(filename: &str, graph: &Graph<(Coord, Direction), i32>) {
    let dot_data = format!("{:?}", Dot::new(&graph));
    let mut file = File::create(std::format!("{}.dot", filename)).expect("Error creating DOT file");
    file.write_all(dot_data.as_bytes())
        .expect("Error writing to DOT file");
    Command::new("sh")
        .arg("-c")
        .arg(std::format!(
            "dot -Tpng {}.dot -o {}.png",
            filename,
            filename
        ))
        .output()
        .expect("failed to execute process");
}

fn parse_graph(input: &str) -> (Graph<(Coord, Direction), i32>, NodeIndex, Vec<NodeIndex>) {
    let mut start_node = None;
    let mut end_nodes = Vec::new();
    let mut graph = DiGraph::new();
    let mut node_indices = HashMap::new();
    let matrix = utils::matrix::parse_matrix(input);
    for (i, row) in (0i32..).zip(matrix.iter()) {
        for (j, value) in (0i32..).zip(row.iter()) {
            for dir in Direction::iterator() {
                match *value {
                    '#' => {}
                    '.' => {
                        node_indices.insert(
                            (Coord::new(i, j), *dir),
                            graph.add_node((Coord::new(i, j), *dir)),
                        );
                    }
                    'E' => {
                        let end = graph.add_node((Coord::new(i, j), *dir));
                        end_nodes.push(end);
                        node_indices.insert((Coord::new(i, j), *dir), end);
                    }
                    'S' => {
                        let start = graph.add_node((Coord::new(i, j), *dir));
                        if *dir == Direction::South {
                            start_node = Some(start);
                        }
                        node_indices.insert((Coord::new(i, j), *dir), start);
                    }
                    _ => {
                        panic!("Invalid cell")
                    }
                }
            }
        }
    }

    let mut edges = Vec::new();

    for node in graph.node_indices() {
        let (c, dir) = graph.node_weight(node).unwrap();
        let c_2 = c + dir.get_vector();
        if let Some(n_2) = node_indices.get(&(c_2, *dir)) {
            edges.push((node, *n_2, 1));
        }

        for left_right_neighbour in dir.get_left_right_neigbhours() {
            if let Some(n_lr) = node_indices.get(&(*c, left_right_neighbour)) {
                edges.push((node, *n_lr, 1000));
            }
        }
    }
    for edge in edges {
        graph.add_edge(edge.0, edge.1, edge.2);
    }
    (graph, start_node.unwrap(), end_nodes)
}

fn part_one(input: &str) -> i32 {
    let (graph, sid, eids) = parse_graph(input);
    let res = petgraph::algo::dijkstra(&graph, sid, None, |e| *e.weight());
    let mut min_score = i32::MAX;
    for eid in eids {
        min_score = min_score.min(res[&eid]);
    }
    min_score
}

#[derive(PartialEq, Eq, Debug)]
struct WorkNode(petgraph::graph::NodeIndex, i32);

impl PartialOrd for WorkNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for WorkNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

fn part_two(input: &str) -> i32 {
    let (graph, sid, eids) = parse_graph(input);
    let mut map = HashMap::new();
    let mut def = HashSet::new();
    def.insert(graph.node_weight(sid).unwrap().0);
    map.insert(sid, (0, def));
    let mut work_vec: BinaryHeap<WorkNode> = BinaryHeap::new();
    work_vec.push(WorkNode(sid, 0));
    while !work_vec.is_empty() {
        let WorkNode(closest_node, dis) = work_vec.pop().unwrap();
        let edges_from_closest = graph.edges_directed(closest_node, petgraph::Direction::Outgoing);
        for edge in edges_from_closest {
            let next_dis = dis + edge.weight();
            let target = edge.target();
            let edge_parents = map.get(&closest_node).unwrap().1.clone();
            let (prev_dis, prev_parents) = map.entry(target).or_insert((i32::MAX, HashSet::new()));
            if *prev_dis > next_dis {
                *prev_dis = next_dis;
                prev_parents.clear();
                prev_parents.insert(graph.node_weight(closest_node).unwrap().0);
                prev_parents.extend(edge_parents);
                work_vec.push(WorkNode(target, next_dis));
            } else if (*prev_dis == next_dis) {
                prev_parents.insert(graph.node_weight(closest_node).unwrap().0);
                prev_parents.extend(edge_parents);
                work_vec.push(WorkNode(target, next_dis));
            } else {
                // noop
            }
        }
    }

    let mut min_score = i32::MAX;
    let mut min_map = 0;
    for eid in eids {
        min_score = min_score.min(map[&eid].0);
        if min_score == map[&eid].0 {
            min_map = map[&eid].1.len() + 1;
        }
    }
    min_map as i32
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_2: &str = include_str!("example_2.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 7036);
        assert_eq!(part_one(EXAMPLE_2), 11048);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 45);
        assert_eq!(part_two(EXAMPLE_2), 64);
    }
}

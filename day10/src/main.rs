use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::process::Command;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn get_neighbour_indices(c: Coord) -> Vec<Coord> {
    let mut res = Vec::with_capacity(4);
    res.push(c+Coord(1, 0));
    res.push(c+Coord(0, 1));
    res.push(c+Coord(-1, 0));
    res.push(c+Coord(0, -1));
    res
}

struct TopographicMap {
    graph: DiGraph<i32, i32>,
    height_map : HashMap<i32, Vec<Coord>>,
    node_indices: HashMap<Coord, petgraph::graph::NodeIndex>,
}

impl TopographicMap {
    fn new(input: &str) -> Self {
        let matrix = utils::parse_matrix(input);
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();
        let mut height_map : HashMap<i32, Vec<Coord>> = HashMap::new();

        matrix.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, char)| {
                let height = char.to_digit(10).unwrap() as i32;
                height_map.entry(height).or_default().push(Coord(i as i32, j as i32));
                node_indices.insert(Coord(i as i32, j as i32), graph.add_node(height));
            })
        });

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let coord = Coord(i as i32, j as i32);
                let coord_id = node_indices.get(&coord).unwrap();
                let height = matrix[i][j].to_digit(10).unwrap() as i32;
                for neighbour in get_neighbour_indices(coord) {
                    let neighbour_node_id = node_indices.get(&neighbour);
                    if neighbour_node_id.is_none() {
                        continue;
                    }
                    let neighbour_weight = *graph.node_weight(*neighbour_node_id.unwrap()).unwrap();
                    if height - neighbour_weight == 1 {
                        graph.add_edge(*neighbour_node_id.unwrap(), *coord_id, 1);
                    }
                }
            }
        }
        Self { graph, height_map, node_indices }
    }

    fn export_to_png(&self, filename: &str) {
        let dot_data = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        let mut file = File::create(std::format!("{}.dot", filename)).expect("Error creating DOT file");
        file.write_all(dot_data.as_bytes()).expect("Error writing to DOT file");
        Command::new("sh")
            .arg("-c")
            .arg(std::format!("dot -Tpng {}.dot -o {}.png", filename, filename))
            .output()
            .expect("failed to execute process");
    }
}


fn part_one(input: &str) -> u64 {
    let map = TopographicMap::new(input);
    let mut sum = 0;
    for trailhead in map.height_map.get(&0).unwrap_or(&vec![]) {
        let trailhead_node_id = map.node_indices.get(&trailhead).unwrap();
        assert_eq!(map.graph.node_weight(*trailhead_node_id), Some(&0));
        let res = petgraph::algo::dijkstra(&map.graph, *trailhead_node_id, None, |_| 1);
        let number_of_trails = res.iter().filter(|(_, l)| **l == 9).count();
        sum += number_of_trails as u64;
    }
    sum
}

fn part_two(input: &str) -> u64 {
    let map = TopographicMap::new(input);
    let mut sum = 0;
    let mountain_tops = map.height_map.get(&9).unwrap();
    for trailhead in map.height_map.get(&0).unwrap() {
        let trailhead_node_id = map.node_indices.get(&trailhead).unwrap();
        assert_eq!(map.graph.node_weight(*trailhead_node_id), Some(&0));
        for mountain_top in mountain_tops {
            let mountain_top_node_id = map.node_indices.get(&mountain_top).unwrap();
            assert_eq!(map.graph.node_weight(*mountain_top_node_id), Some(&9));
            sum += petgraph::algo::simple_paths::all_simple_paths::<Vec<_>, _>(&map.graph, *trailhead_node_id, *mountain_top_node_id, 8, None).count();
        }
    }
    sum as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_SMALL: &str = include_str!("example_small.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE_SMALL), 2);
        assert_eq!(part_one(EXAMPLE), 36);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 81);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

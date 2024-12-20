use petgraph::dot::{Config, Dot};
use petgraph::graph::Graph;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::process::Command;
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn get_neighbours(c: Coord) -> Vec<Coord> {
    let mut res = Vec::with_capacity(4);
    res.push(c + Coord(1, 0));
    res.push(c + Coord(0, 1));
    res.push(c + Coord(-1, 0));
    res.push(c + Coord(0, -1));
    res
}

fn get_corners(c: Coord) -> [[Coord; 3]; 4] {
    [
        [c + Coord(1, 0), c + Coord(1, 1), c + Coord(0, 1)],
        [c + Coord(0, 1), c + Coord(-1, 1), c + Coord(-1, 0)],
        [c + Coord(-1, 0), c + Coord(-1, -1), c + Coord(0, -1)],
        [c + Coord(0, -1), c + Coord(1, -1), c + Coord(1, 0)],
    ]
}

struct Fields {
    graph: Graph<(char, Coord), i32>,
    node_indices: HashMap<Coord, petgraph::graph::NodeIndex>,
}

impl Fields {
    fn new(input: &str) -> Self {
        let matrix = utils::matrix::parse_matrix(input);
        let mut graph = Graph::new();
        let mut node_indices = HashMap::new();

        matrix.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, char)| {
                node_indices.insert(
                    Coord(i as i32, j as i32),
                    graph.add_node((*char, Coord(i as i32, j as i32))),
                );
            })
        });

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                let coord = Coord(i as i32, j as i32);
                let coord_id = node_indices.get(&coord).unwrap();
                let crop_type = matrix[i][j];
                for neighbour in get_neighbours(coord) {
                    let neighbour_node_id = node_indices.get(&neighbour);
                    if neighbour_node_id.is_none() {
                        continue;
                    }
                    let (neighbour_crop_type, _) =
                        *graph.node_weight(*neighbour_node_id.unwrap()).unwrap();
                    if crop_type == neighbour_crop_type {
                        graph.add_edge(*neighbour_node_id.unwrap(), *coord_id, 1);
                    }
                }
            }
        }
        Self {
            graph,
            node_indices,
        }
    }

    fn export_to_png(&self, filename: &str) {
        let dot_data = format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        );
        let mut file =
            File::create(std::format!("{}.dot", filename)).expect("Error creating DOT file");
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
}

fn part_one(input: &str) -> u64 {
    let map = Fields::new(input);
    let crop_fields = petgraph::algo::tarjan_scc(&map.graph);
    let mut sum = 0;
    for crop_field in crop_fields {
        let mut area = 0;
        let mut perimeter = 0;
        for crop_node_index in crop_field {
            let crop_node_edges = map.graph.edges(crop_node_index);
            perimeter += 4 - crop_node_edges.count();
            area += 1;
        }
        sum += area * perimeter;
    }
    //map.export_to_png("day12");
    sum as u64
}

fn part_two(input: &str) -> u64 {
    let map = Fields::new(input);
    let crop_fields = petgraph::algo::tarjan_scc(&map.graph);
    let mut sum = 0;
    for crop_field in crop_fields {
        let mut area = 0;
        let mut break_points = HashSet::new();
        let mut trickies = 0;
        let coords_of_crop_field = crop_field
            .iter()
            .map(|c| map.graph.node_weight(*c).unwrap().1)
            .collect::<Vec<_>>();
        for crop_node_index in crop_field {
            let crop_node_coord = map.graph.node_weight(crop_node_index).unwrap().1;
            get_corners(crop_node_coord).iter().for_each(|c| {
                let is_0_crop = coords_of_crop_field.contains(&c[0]);
                let is_1_crop = coords_of_crop_field.contains(&c[1]);
                let is_2_crop = coords_of_crop_field.contains(&c[2]);

                let breaks = match (is_0_crop, is_1_crop, is_2_crop) {
                    (false, false, true) => false,
                    (true, false, false) => false,
                    (true, true, true) => false,
                    (false, true, false) => {
                        trickies += 1;
                        true
                    }
                    (_, _, _) => true,
                };
                let x = c[0].0 + c[1].0 + c[2].0 + crop_node_coord.0;
                let y = c[0].1 + c[1].1 + c[2].1 + crop_node_coord.1;
                if breaks {
                    break_points.insert((x, y));
                }
            });

            area += 1;
        }
        assert!(break_points.len() >= 4);
        sum += area * (break_points.len() + trickies / 2) as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    const EXAMPLE_SMALL: &str = include_str!("example_small.txt");
    const EXAMPLE_E: &str = include_str!("example_e.txt");
    const EXAMPLE_TRICKY: &str = include_str!("example_tricky.txt");

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE_SMALL), 140);
        assert_eq!(part_one(EXAMPLE), 1930);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE_SMALL), 80);
        assert_eq!(part_two(EXAMPLE_E), 236);
        assert_eq!(part_two(EXAMPLE), 1206);
        assert_eq!(part_two(EXAMPLE_TRICKY), 368);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

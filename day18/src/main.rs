use petgraph::visit::NodeRef;
use std::collections::HashMap;
use utils::coord::Coord;

fn parse_bytes(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(",")
                .map_or((0, 0), |(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
            Coord::new(x, y)
        })
        .collect()
}

fn create_graph(
    matrix_size: i32,
) -> (
    petgraph::Graph<Coord, i32>,
    HashMap<Coord, petgraph::graph::NodeIndex>,
    petgraph::graph::NodeIndex,
    petgraph::graph::NodeIndex,
) {
    let mut graph = petgraph::Graph::new();
    let mut nodes = HashMap::new();

    let start_node = graph.add_node(Coord { x: 0, y: 0 });
    let end_node = graph.add_node(Coord {
        x: matrix_size - 1,
        y: matrix_size - 1,
    });

    nodes.insert(Coord::new(0, 0), start_node);
    nodes.insert(Coord::new(matrix_size - 1, matrix_size - 1), end_node);

    for i in 0..matrix_size {
        for j in 0..matrix_size {
            let coord = Coord::new(i, j);
            if (i, j) != (0, 0) && (i, j) != (matrix_size - 1, matrix_size - 1) {
                nodes.insert(coord, graph.add_node(coord));
            }
        }
    }

    for (c, node) in &nodes {
        for neighbour in c.get_neighbours() {
            if let Some(neighbour_node) = nodes.get(&neighbour) {
                graph.update_edge(*node, *neighbour_node, 1);
            }
        }
    }

    (graph, nodes, start_node, end_node)
}

fn part_one(input: &str, matrix_size: i32, first_n_bytes: usize) -> i32 {
    let bytes = parse_bytes(input);
    let (mut graph, _nodes, start_node, end_node) = create_graph(matrix_size);
    graph.retain_nodes(|graph, node_index| {
        let node_coord = graph
            .node_weight(node_index)
            .expect("node should be present");
        !bytes[..first_n_bytes].contains(node_coord)
    });

    let res = petgraph::algo::dijkstra(&graph, start_node, Some(end_node), |_| 1);

    *res.get(&end_node).unwrap()
}

fn part_two(input: &str, matrix_size: i32) -> Coord {
    let bytes = parse_bytes(input);
    let (mut graph, _nodes, start_node, end_node) = create_graph(matrix_size);

    for byte in bytes {
        graph.retain_nodes(|graph, node_index| graph.node_weight(node_index) != Some(&byte));
        if !petgraph::algo::has_path_connecting(&graph, start_node, end_node, None) {
            return byte;
        }
    }
    panic!("No path found");
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE, 7, 12), 22);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE, 7), Coord::new(6, 1));
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!(
        "{} part one: {}",
        env!("CARGO_PKG_NAME"),
        part_one(INPUT, 71, 1024)
    );
    println!(
        "{} part two: {:?}",
        env!("CARGO_PKG_NAME"),
        part_two(INPUT, 71)
    );
}

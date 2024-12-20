use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeIdentifiers;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

fn parse(input: &str) -> (petgraph::Graph<u32, ()>, Vec<Vec<u32>>) {
    let (page_ordering_input, update_pages_input) = input.split_once("\n\n").unwrap();
    (
        parse_page_ordering_rules(page_ordering_input),
        parse_update_pages(update_pages_input),
    )
}

fn parse_page_ordering_rules(input: &str) -> petgraph::Graph<u32, ()> {
    let edges: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(a_str, b_str)| {
                    (b_str.parse::<u32>().unwrap(), a_str.parse::<u32>().unwrap())
                })
                .unwrap()
        })
        .collect();
    let mut graph = petgraph::Graph::<u32, ()>::new();
    let mut node_indices = HashMap::<u32, NodeIndex>::new();
    for edge in edges {
        if !node_indices.contains_key(&edge.0) {
            node_indices.insert(edge.0, graph.add_node(edge.0));
        }
        if !node_indices.contains_key(&edge.1) {
            node_indices.insert(edge.1, graph.add_node(edge.1));
        }
        graph.add_edge(node_indices[&edge.0], node_indices[&edge.1], ());
    }
    graph
}

fn parse_update_pages(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse::<u32>().ok())
                .collect()
        })
        .collect()
}

fn is_sorted(
    data: &[u32],
    graph: &petgraph::Graph<u32, ()>,
    graph_indices: &HashMap<u32, NodeIndex>,
) -> bool {
    data.windows(2)
        .all(|w| compare(w[0], w[1], graph, graph_indices) == Ordering::Less)
}

fn compare(
    a: u32,
    b: u32,
    graph: &petgraph::Graph<u32, ()>,
    graph_indices: &HashMap<u32, NodeIndex>,
) -> Ordering {
    let node_0 = graph_indices.get(&a).unwrap();
    let node_1 = graph_indices.get(&b).unwrap();
    if petgraph::algo::has_path_connecting(&graph, *node_0, *node_1, None) {
        return Ordering::Greater;
    }
    Ordering::Less
}

fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    let (ordering_rules, update_pages) = parse(input);

    for update_page in &update_pages {
        let mut curr_graph = ordering_rules.clone();
        curr_graph.retain_nodes(|fgr, node_index| {
            update_page.contains(fgr.node_weight(node_index).unwrap_or(&100))
        });

        let mut curr_indices = HashMap::new();
        for node_id in curr_graph.node_identifiers() {
            curr_indices.insert(curr_graph[node_id], node_id);
        }

        if is_sorted(update_page, &curr_graph, &curr_indices) {
            let middle_index = update_page.len() / 2;
            sum += update_page.get(middle_index).unwrap();
        }
    }
    sum as i32
}

fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    let (ordering_rules, update_pages) = parse(input);

    for update_page in &update_pages {
        let mut curr_graph = ordering_rules.clone();
        curr_graph.retain_nodes(|fgr, node_index| {
            update_page.contains(fgr.node_weight(node_index).unwrap_or(&100))
        });

        let mut curr_indices = HashMap::new();
        for node_id in curr_graph.node_identifiers() {
            curr_indices.insert(curr_graph[node_id], node_id);
        }

        if !is_sorted(update_page, &curr_graph, &curr_indices) {
            let mut sorted_page = update_page.clone();
            sorted_page.sort_by(|a, b| compare(*a, *b, &curr_graph, &curr_indices));
            let middle_index = sorted_page.len() / 2;
            sum += sorted_page.get(middle_index).unwrap();
        }
    }
    sum as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("example.txt");
    #[test]
    fn example_part_one() {
        assert_eq!(part_one(EXAMPLE), 143);
    }

    #[test]
    fn example_part_two() {
        assert_eq!(part_two(EXAMPLE), 123);
    }
}

fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part two: {}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}

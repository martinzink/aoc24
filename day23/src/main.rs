use std::collections::{HashMap, HashSet};
use petgraph::visit::{EdgeRef};
use petgraph::graph::NodeIndex;

fn parse(input: &str) -> (petgraph::graph::UnGraph<&str, i32>, HashMap<&str, NodeIndex>) {
    let mut graph = petgraph::graph::Graph::new_undirected();
    let mut nodes = HashMap::new();
    input.lines().for_each(|l| {
        let (a,b) = l.split_once("-").unwrap();
        if !nodes.contains_key(&a) {
            nodes.insert(a, graph.add_node(a));
        }
        if !nodes.contains_key(&b) {
            nodes.insert(b, graph.add_node(b));
        }
        graph.add_edge(*nodes.get(a).unwrap(), *nodes.get(b).unwrap(), 1);
    });
    (graph, nodes)
}

fn part_one(input: &str) -> u32 {
    let (graph, nodes) = parse(input);
    let mut res = HashSet::new();
    nodes.iter().filter(|(name, node)| name.starts_with('t')).for_each(|(name, node)| {
        let edges = graph.edges(*node).collect::<Vec<_>>();
        for i in 0..edges.len() {
            for j in 0..edges.len() {
                if i == j {
                    continue;
                }
                let neighbour_one = edges[i].target();
                let neighbour_two = edges[j].target();
                if graph.edges_connecting(neighbour_one, neighbour_two).count() != 0 {
                    let mut v = vec![*node, neighbour_one, neighbour_two];
                    v.sort();
                    res.insert(v);
                }
            }
        }
    });
    res.len() as u32
}

fn part_two(input: &str) -> String {
    let (graph, nodes) = parse(input);
    let max_clieche = start_bron_kerbosch(&graph);
    let mut res = max_clieche.iter().map(|node_index| { *graph.node_weight(*node_index).unwrap() }).collect::<Vec<_>>();
    res.sort();
    res.join(",")
}

fn bron_kerbosch(graph: &petgraph::graph::UnGraph<&str, i32>, mut r : HashSet<NodeIndex>, mut p : HashSet<NodeIndex>, mut x : HashSet<NodeIndex>, res : &mut Vec::<HashSet<NodeIndex>>)  {
    if p.is_empty() && x.is_empty() {
        res.push(r);
        return;
    }
    for v in p.clone() {
        let new_r = r.union(&HashSet::from_iter(vec![v])).map(|a|*a).collect::<HashSet<NodeIndex>>();
        let neighbours_of_v = graph.neighbors(v).collect::<HashSet<_>>();
        let new_p = p.intersection(&neighbours_of_v).map(|a|*a).collect::<HashSet<_>>();
        let new_x = x.intersection(&neighbours_of_v).map(|a|*a).collect::<HashSet<_>>();
        bron_kerbosch(graph, new_r, new_p, new_x, res);
        p.remove(&v);
        x.insert(v);
    }
}

fn start_bron_kerbosch(graph: &petgraph::graph::UnGraph<&str, i32>) -> HashSet<NodeIndex> {
    let r : HashSet<NodeIndex> = HashSet::new();
    let x : HashSet<NodeIndex> = HashSet::new();
    let p : HashSet<NodeIndex> = graph.node_indices().collect::<HashSet<_>>();
    let mut res = Vec::new();
    bron_kerbosch(&graph, r, p, x, &mut res);
    res.iter().max_by(|a,b | a.len().cmp(&b.len())).unwrap().clone()
}


fn main() {
    const INPUT: &str = include_str!("input.txt");
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_one(INPUT));
    println!("{} part one: {:?}", env!("CARGO_PKG_NAME"), part_two(INPUT));
}
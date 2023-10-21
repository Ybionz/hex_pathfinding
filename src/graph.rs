use crate::hex::Hex;
use petgraph::data::Build;
use petgraph::prelude::GraphMap;
use petgraph::prelude::UnGraphMap;
use petgraph::Undirected;
use rand::{thread_rng, Rng};

pub fn hex_graph_with_random_remove(
    cols: i32,
    rows: i32,
    removes: i32,
) -> GraphMap<Hex, i32, Undirected> {
    let mut graph = create_hex_nodes(rows, cols);
    create_edges(&mut graph);

    for _ in 0..removes {
        let node_index = random_node(rows, cols);
        remove_random_edge_from_node(&mut graph, node_index);
    }
    graph
}

fn random_node(rows: i32, cols: i32) -> Hex {
    let mut rng = thread_rng();
    let random_row = rng.gen_range(0..rows);
    let random_col = rng.gen_range(0..cols);
    return Hex::new(random_col, random_row);
}

fn remove_random_edge_from_node(graph: &mut GraphMap<Hex, i32, Undirected>, node: Hex) {
    let upper_bound = graph.edges(node).count();
    if upper_bound == 0 {
        return;
    }
    let mut rng = thread_rng();
    let rng_edge = rng.gen_range(0..upper_bound);

    match graph.edges(node).into_iter().nth(rng_edge) {
        Some((from, to, _)) => graph.remove_edge(from, to),
        None => return,
    };
}

fn create_hex_nodes(rows: i32, cols: i32) -> GraphMap<Hex, i32, Undirected> {
    let mut g: GraphMap<Hex, i32, Undirected> = UnGraphMap::new();
    (0..rows).into_iter().for_each(|row| {
        (0..cols).into_iter().for_each(|col| {
            g.add_node(Hex::new(col, row));
        })
    });
    g
}

pub fn remove_all_edges_for_node(graph: &mut GraphMap<Hex, i32, Undirected>, hex: Hex) {
    graph
        .clone()
        .edges(hex)
        .clone()
        .into_iter()
        .for_each(|edge| {
            let _ = &graph.remove_edge(edge.0, edge.1);
        });

    // (&graph)
    //     .edges(hex)
    //     // .for_each(|edge| {
    //     .for_each(|(node1, node2, _)| {
    //         &graph.remove_edge(node1, node2);
    //     });
    // graph
}

fn create_edges(graph: &mut GraphMap<Hex, i32, Undirected>) {
    let edge_list: Vec<(Hex, Vec<Hex>)> = graph
        .nodes()
        .into_iter()
        .map(|node| {
            (
                node,
                node.neighbors()
                    .iter()
                    .flatten()
                    .filter(|&n| graph.contains_node(*n))
                    .map(|&n| n)
                    .collect(),
            )
        })
        .collect();
    edge_list.into_iter().for_each(|(node, neighbors)| {
        neighbors.into_iter().for_each(|neighbor| {
            graph.update_edge(node, neighbor, 1);
        });
    });
}

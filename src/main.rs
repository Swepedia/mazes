use crate::node::*;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use rand::seq::SliceRandom;

mod node;

type Maze = Graph<Node, u32, Undirected>;

fn main() {
    let maze = binary_tree(create_graph(8, 8));
}

fn create_graph(rows: u32, columns: u32) -> Maze {
    let mut graph = Graph::new_undirected();
    for i in 0..rows {
        for j in 0..columns {
            let east;
            let west;
            let north;
            let south;
            if i == 0 {
                north = None;
            } else {
                north = Some(NodeIndex::new(((i - 1) * columns + j) as usize));
            }
            if i == rows - 1 {
                south = None;
            } else {
                south = Some(NodeIndex::new(((i + 1) * columns + j) as usize));
            }
            if j == 0 {
                west = None;
            } else {
                west = Some(NodeIndex::new((i * columns + j - 1) as usize));
            }
            if j == rows - 1 {
                east = None;
            } else {
                east = Some(NodeIndex::new((i * columns + j + 1) as usize))
            }
            graph.add_node(Node::new(i, j, north, east, south, west));
        }
    }
    graph
}

fn binary_tree(input_graph: Maze) -> Maze {
    let mut graph = input_graph;
    for i in graph.node_indices() {
        let mut neighbors = Vec::<NodeIndex>::new();
        if let Some(north) = graph[i].north {
            neighbors.push(north);
        }
        if let Some(east) = graph[i].east {
            neighbors.push(east);
        }

        if let Some(neighbor) = neighbors.choose(&mut rand::thread_rng()) {
            graph.add_edge(i, *neighbor, 1);
        }
    }
    graph
}

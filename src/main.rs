use crate::node::*;
use petgraph::{Graph, Undirected};

mod node;

fn main() {
    println!("Hello, world!");
}

fn create_graph(rows: i32, columns: i32) -> Graph<Node, u32, Undirected> {
    let mut graph = Graph::new_undirected();
    for i in 0..rows {
        let east;
        let west;
        if i == 0 {
            west = None;
        } else {
            west = Some(i - 1);
        }
        if i == rows - 1 {
            east = None;
        } else {
            east = Some(i + 1)
        }

        for j in 0..columns {
            let north;
            let south;
            if j == 0 {
                north = None;
            } else {
                north = Some(j - 1);
            }
            if j == columns - 1 {
                south = None;
            } else {
                south = Some(j + 1)
            }
            graph.add_node(Node::new(i, j, north, east, south, west));
        }
    }
    graph
}

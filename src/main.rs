use crate::grid::*;
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;

mod grid;
mod node;

fn main() {
    let maze = binary_tree(Grid::new(7, 5));
    maze.print_ascii();
}

fn binary_tree(input_graph: Grid) -> Grid {
    let mut graph = input_graph;
    for i in graph.graph.node_indices() {
        let mut neighbors = Vec::<NodeIndex>::new();
        if let Some(north) = graph.graph[i].north {
            neighbors.push(north);
        }
        if let Some(east) = graph.graph[i].east {
            neighbors.push(east);
        }

        if let Some(neighbor) = neighbors.choose(&mut rand::thread_rng()) {
            graph.graph.add_edge(i, *neighbor, 1);
        }
    }
    graph
}

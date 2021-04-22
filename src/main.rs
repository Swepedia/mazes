use crate::grid::*;
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;

mod grid;
mod node;

fn main() {
    let mut graph = Grid::new(30, 11);
    let maze = binary_tree(&mut graph);
    maze.print_ascii();
}

fn binary_tree(input_graph: &mut Grid) -> &mut Grid {
    for i in input_graph.graph.node_indices() {
        let mut neighbors = Vec::<NodeIndex>::new();
        if let Some(north) = input_graph.graph[i].north {
            neighbors.push(north);
        }
        if let Some(east) = input_graph.graph[i].east {
            neighbors.push(east);
        }

        if let Some(neighbor) = neighbors.choose(&mut rand::thread_rng()) {
            input_graph.graph.add_edge(i, *neighbor, 1);
        }
    }
    input_graph
}

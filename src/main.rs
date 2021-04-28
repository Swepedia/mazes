use crate::grid::*;
use petgraph::graph::NodeIndex;
use rand::seq::SliceRandom;

mod grid;
mod node;

fn main() {
    let mut graph = Grid::new(40, 40);
    //let maze = binary_tree(&mut graph);
    let maze = aldous_broder(&mut graph);
    maze.print_ascii();
    //maze.print_png(15);
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

fn sidewinder(input_graph: &mut Grid) -> &mut Grid {
    for row in input_graph.each_row() {
        let mut run = Vec::<NodeIndex>::new();
        for cell in row {
            run.push(cell);
            let at_eastern_boundary = input_graph.graph[cell].east.is_none();
            let at_northern_boundary = input_graph.graph[cell].north.is_none();

            let should_close =
                at_eastern_boundary || (!at_northern_boundary && rand::random::<bool>());

            if should_close {
                if let Some(cut) = run.choose(&mut rand::thread_rng()) {
                    if let Some(north_cut) = input_graph.graph[*cut].north {
                        input_graph.graph.add_edge(*cut, north_cut, 1);
                    }
                }
                run.clear();
            } else {
                input_graph
                    .graph
                    .add_edge(cell, input_graph.graph[cell].east.unwrap(), 1);
            }
        }
    }
    input_graph
}

fn aldous_broder(input_graph: &mut Grid) -> &mut Grid {
    let all_cells: Vec<NodeIndex> = input_graph.graph.node_indices().collect();
    let mut current_cell = *all_cells.choose(&mut rand::thread_rng()).unwrap();
    let mut unvisited = all_cells.len() - 1;

    while unvisited > 0 {
        let mut neighbors = Vec::<NodeIndex>::new();
        if let Some(north) = input_graph.graph[current_cell].north {
            neighbors.push(north);
        }
        if let Some(east) = input_graph.graph[current_cell].east {
            neighbors.push(east);
        }
        if let Some(south) = input_graph.graph[current_cell].south {
            neighbors.push(south);
        }
        if let Some(west) = input_graph.graph[current_cell].west {
            neighbors.push(west);
        }

        let neighbor = *neighbors.choose(&mut rand::thread_rng()).unwrap();
        if input_graph.graph.neighbors(neighbor).count() == 0 {
            input_graph.graph.add_edge(current_cell, neighbor, 1);
            unvisited -= 1;
        }
        current_cell = neighbor;
    }

    input_graph
}

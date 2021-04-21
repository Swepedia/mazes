use petgraph::{Graph, Undirected};

pub struct Grid {
    pub rows: u32,
    pub columns: u32,
    pub graph: Graph<Node, u32, Undirected>,
}

impl Grid {
    pub fn new(rows: u32, columns: u32) -> Self {
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

        Self {
            rows,
            columns,
            graph,
        }
    }

    pub fn print_ascii(&self) {
        // Top row
        for i in 0..self.columns {
            print!("+---");
        }
        println!("+");

        // Body
        for i in 0..self.rows {
            for j in 0..self.columns {
                let mut west = "|";
                let mut east = "|";
                let mut south = "___";
                if self.graph[NodeIndex::new(i * self.columns + j)].west == None {
                    west = " ";
                }
                if self.graph[NodeIndex::new(i * self.columns + j)].east == None {
                    east = " ";
                }
                if self.graph[NodeIndex::new(i * self.columns + j)].south == None {
                    south = "   ";
                }

                print!(west + south + east);
            }
            println!("");
        }
    }
}

use crate::node::*;
use image::{GrayImage, Luma};
use petgraph::graph::NodeIndex;
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
                if j == columns - 1 {
                    east = None;
                } else {
                    east = Some(NodeIndex::new((i * columns + j + 1) as usize))
                }
                graph.add_node(Node::new(j, i, north, east, south, west));
            }
        }

        Self {
            rows,
            columns,
            graph,
        }
    }

    pub fn each_row(&self) -> Vec<Vec<NodeIndex>> {
        let mut row_list = Vec::<Vec<NodeIndex>>::new();
        for i in 0..self.rows {
            let mut row = Vec::<NodeIndex>::new();
            for j in 0..self.columns {
                row.push(NodeIndex::new((i * self.columns + j) as usize));
            }
            row_list.push(row);
        }
        row_list
    }

    pub fn print_ascii(&self) {
        // Top row
        for _ in 0..self.columns {
            print!("+---");
        }
        println!("+");

        // Body
        for i in self.graph.node_indices() {
            let mut west = "";
            let mut east = "|";
            let mut south = "___";
            let has_west = self.graph[i].west.is_some();
            let has_east = self.graph[i].east.is_some();
            let has_south = self.graph[i].south.is_some();
            for j in self.graph.neighbors(i) {
                if has_east && self.graph[i].east.unwrap() == j {
                    east = " ";
                } else if has_south && self.graph[i].south.unwrap() == j {
                    south = "   ";
                }
            }
            if !has_west {
                west = "|";
            }
            if east == " " && south == "___" {
                east = "_";
            }

            print!("{}{}{}", west, south, east);
            if !has_east {
                println!("");
            }
        }
    }

    pub fn print_png(&self, size: u32) {
        let cell_size = size;
        let img_width = self.columns * cell_size;
        let img_height = self.rows * cell_size;

        // Initialize image as white sheet
        let mut img = GrayImage::from_fn(img_width + 1, img_height + 1, |_, _| Luma([255u8]));

        // Draw outside borders
        for i in 0..img_width {
            img.put_pixel(i, 0, Luma([0u8]));
            img.put_pixel(i, img_height, Luma([0u8]));
        }
        for i in 0..img_height {
            img.put_pixel(0, i, Luma([0u8]));
            img.put_pixel(img_width, i, Luma([0u8]));
        }

        // Add the cells to the image
        for i in self.graph.node_indices() {
            let x1 = self.graph[i].x * cell_size;
            let y1 = self.graph[i].y * cell_size;
            let x2 = (self.graph[i].x + 1) * cell_size;
            let y2 = (self.graph[i].y + 1) * cell_size;
            let has_east = self.graph[i].east.is_some();
            let has_south = self.graph[i].south.is_some();

            // Draw if cells aren't connected
            let mut local_neighbors = Vec::<NodeIndex>::new();
            for j in self.graph.neighbors(i) {
                local_neighbors.push(j);
            }
            if has_east && !local_neighbors.contains(&self.graph[i].east.unwrap()) {
                for k in 0..cell_size {
                    img.put_pixel(x2, y1 + k, Luma([0u8]));
                }
            }
            if has_south && !local_neighbors.contains(&self.graph[i].south.unwrap()) {
                for k in 0..cell_size {
                    img.put_pixel(x1 + k, y2, Luma([0u8]));
                }
            }
        }
        img.save("maze.png").expect("Failed to write");
    }
}

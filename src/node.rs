use petgraph::graph::NodeIndex;

pub struct Node {
    pub x: u32,
    pub y: u32,
    pub north: Option<NodeIndex>,
    pub east: Option<NodeIndex>,
    pub south: Option<NodeIndex>,
    pub west: Option<NodeIndex>,
}

impl Node {
    pub fn new(
        x: u32,
        y: u32,
        north: Option<NodeIndex>,
        east: Option<NodeIndex>,
        south: Option<NodeIndex>,
        west: Option<NodeIndex>,
    ) -> Self {
        Self {
            x,
            y,
            north,
            east,
            south,
            west,
        }
    }
}

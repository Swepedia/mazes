pub struct Node {
    x: i32,
    y: i32,
    north: Option<i32>,
    east: Option<i32>,
    south: Option<i32>,
    west: Option<i32>,
}

impl Node {
    pub fn new(
        x: i32,
        y: i32,
        north: Option<i32>,
        east: Option<i32>,
        south: Option<i32>,
        west: Option<i32>,
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

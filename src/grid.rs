use crate::node::*;
use std::rc::Rc;

pub struct Grid<'a> {
    pub rows: usize,
    pub columns: usize,
    grid: Vec<Vec<Rc<Node>>>,
}

impl<'a> Grid<'a> {
    pub fn new(rows: usize, columns: usize) -> Self {
        //let mut grid = GridVec::new(rows, columns);
        let mut grid = Vec::new();

        for i in 0..rows {
            grid.push(create_row(i, columns));
            //for j in 0..columns {
            //    grid.get(i, j) = &mut Some(&Node::new(i, j));
            // grid.get(i, j).map(|node| {
            //     node.row = i;
            //     node.column = j;
            // });
            //}
        }

        for i in 0..rows {
            for j in 0..columns {
                let mut is_north = false;
                let mut is_south = false;
                let mut is_east = false;
                let mut is_west = false;

                if let Some(_) = grid.get(i - 1) {
                    is_north = true;
                }
                if let Some(_) = grid.get(i + 1) {
                    is_south = true;
                }
                if let Some(_) = grid[i].get(j + 1) {
                    is_east = true;
                }
                if let Some(_) = grid[i].get(j - 1) {
                    is_west = true;
                }

                if is_north {
                    let cell = grid[i - 1][j].clone();
                    grid[i][j].set_north(cell);
                }
                if is_south {
                    let cell = grid[i + 1][j].clone();
                    grid[i][j].set_south(cell);
                }
                if is_east {
                    let cell = grid[i][j + 1].clone();
                    grid[i][j].set_east(cell);
                }
                if is_west {
                    let cell = grid[i][j - 1].clone();
                    grid[i][j].set_west(cell);
                }
                //grid.get_mut(i - 1)
                //    .map(|row| row.get(j).map(|cell| grid[i][j].set_north(cell)));
                //grid.get_mut(i + 1)
                //    .map(|row| row.get(j).map(|cell| grid[i][j].set_south(cell)));
                //grid.get_mut(i)
                //    .map(|row| row.get(j + 1).map(|cell| grid[i][j].set_east(cell)));
                //grid.get_mut(i)
                //    .map(|row| row.get(j - 1).map(|cell| grid[i][j].set_west(cell)));
                //grid.get(row - 1, column).map(|cell| {
                //    grid.get(row, column).unwrap().north = Some(cell);
                //});
                //grid.get(row + 1, column).map(|cell| {
                //    grid.get(row, column).unwrap().south = Some(cell);
                //});
                //grid.get(row, column + 1).map(|cell| {
                //    grid.get(row, column).unwrap().east = Some(cell);
                //});
                //grid.get(row, column - 1).map(|cell| {
                //    grid.get(row, column).unwrap().west = Some(cell);
                //});
            }
        }

        Self {
            rows,
            columns,
            grid,
        }
    }
}

fn create_row(row_num: usize, columns: usize) -> Vec<Node<'static>> {
    let mut row = Vec::new();
    for column in 0..columns {
        row.push(Node::new(row_num, column));
    }

    row
}
pub struct GridVec<'a> {
    array: Vec<Vec<Option<Node<'a>>>>,
}

impl<'a> GridVec<'a> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            array: vec![vec![None; columns]; rows],
        }
    }

    pub fn get(&self, row: usize, column: usize) -> Option<&Node> {
        if row > 0 && row < self.array.len() {
            if column > 0 && column < self.array.get(0).unwrap().len() {
                self.array[row][column].as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }
}

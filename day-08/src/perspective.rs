
use super::forest::Forest;

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

/// A Persepctive is a mapping onto a Forest, where each tree of the
/// forest corresponds to a visibility metric of the perspective.
///
/// A visibility metric can range from [0, 4). For each direction
/// that a tree is visible from, it's corresponding visibility metric
/// is increased by one.
#[derive(Debug)]
pub struct Perspective {
    row_size: usize,
    col_size: usize,
    visibility: Vec<u8>,
}

impl From<&Forest> for Perspective {
    fn from(forest: &Forest) -> Self {
        Perspective {
            row_size: forest.row_size(),
            col_size: forest.col_size(),
            visibility: vec![0; forest.len()],
        }
    }
}

impl Perspective {
    fn get(&self, row: usize, col: usize) -> Option<&u8> {
        let index = self.row_size * row + col;
        self.visibility.get(index)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut u8> {
        let index = self.row_size * row + col;
        self.visibility.get_mut(index)
    }

    pub fn view_from_direction(&mut self, forest: &Forest, direction: Direction) {
        match direction {
            Direction::North => {
                for col in 0..self.col_size {
                    let mut highest_tree: u8 = 0;
                    for row in 0..self.row_size {
                        if forest.viewable(row, col, &mut highest_tree) {
                            let visible = self.get_mut(row, col).expect("in-bounds visiblity");
                            *visible += 1;
                        }
                    }
                }
            }
            Direction::East => {
                for row in 0..self.row_size {
                    let mut highest_tree: u8 = 0;
                    for col in (0..self.col_size).rev() {
                        if forest.viewable(row, col, &mut highest_tree) {
                            let visible = self.get_mut(row, col).expect("in-bounds visiblity");
                            *visible += 1;
                        }
                    }
                }
            }
            Direction::South => {
                for col in 0..self.col_size {
                    let mut highest_tree: u8 = 0;
                    for row in (0..self.row_size).rev() {
                        if forest.viewable(row, col, &mut highest_tree) {
                            let visible = self.get_mut(row, col).expect("in-bounds visiblity");
                            *visible += 1;
                        }
                    }
                }
            }
            Direction::West => {
                for row in 0..self.row_size {
                    let mut highest_tree: u8 = 0;
                    for col in 0..self.col_size {
                        if forest.viewable(row, col, &mut highest_tree) {
                            let visible = self.get_mut(row, col).expect("in-bounds visiblity");
                            *visible += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn total_visible_trees(&self) -> u32 {
        let mut total = (2 * self.row_size + 2 * self.col_size - 4) as u32;
        for row in 1..(self.row_size-1) {
            for col in 1..(self.col_size-1) {
                if *self.get(row, col).unwrap() > 0 {
                    total += 1;
                }
            }
        }
        total
    }
}
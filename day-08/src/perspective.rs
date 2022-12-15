
use super::forest::Forest;

#[derive(Debug)]
pub struct Perspective(Vec<Vec<u8>>);

pub enum Direction {
    North,
    East,
    South,
    West
}

impl Perspective {
    pub fn new(width: usize, height: usize) -> Self {
        Perspective(vec![vec![0; width]; height])
    }

    pub fn view_forest_from_direction(&mut self, forest: &Forest, direction: Direction) {
        let view = &mut self.0;

        let width = forest.inner().get(0).unwrap().len();
        let height = forest.inner().len();

        match direction {
            Direction::North => {
                for outer in 0..width {
                    let mut highest_tree: u8 = 0;
                    for inner in 0..height {
                        let tree = forest.inner().get(inner).unwrap().get(outer).unwrap();
                        if *tree > highest_tree {
                            highest_tree = *tree;
                            view[inner][outer] += 1;
                        }
                    }
                }
            }
            Direction::East => {
                for outer in 0..height {
                    let mut highest_tree: u8 = 0;
                    for inner in (0..width).rev() {
                        let tree = forest.inner().get(outer).unwrap().get(inner).unwrap();
                        if *tree > highest_tree {
                            highest_tree = *tree;
                            view[outer][inner] += 1;
                        }
                    }
                }
            }
            Direction::South => {
                for outer in 0..width {
                    let mut highest_tree: u8 = 0;
                    for inner in (0..height).rev() {
                        let tree = forest.inner().get(inner).unwrap().get(outer).unwrap();
                        if *tree > highest_tree {
                            highest_tree = *tree;
                            view[inner][outer] += 1;
                        }
                    }
                }
            }
            Direction::West => {
                for outer in 0..height {
                    let mut highest_tree: u8 = 0;
                    for inner in 0..width {
                        let tree = forest.inner().get(outer).unwrap().get(inner).unwrap();
                        if *tree > highest_tree {
                            highest_tree = *tree;
                            view[outer][inner] += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn count_visible_trees(&self) -> u32 {
        let view = &self.0;

        let width = view.get(0).unwrap().len();
        let height = view.len();

        let mut count = 0;
        for outer in 1..height-1 {
            for inner in 1..width-1 {
                let visibility = view.get(outer).unwrap().get(inner).unwrap();
                if *visibility > 0 {
                    count += 1;
                }
            }
        }

        // count the edges of the forest as visible
        (2 * width + 2 * height - 4) as u32 + count
    }
}
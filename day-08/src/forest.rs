
use std::str::FromStr;

/// A Forest is a 2D map of trees heights. The height of a tree can range
/// from [0, 9].
#[derive(Debug)]
pub struct Forest {
    row_size: usize,
    col_size: usize,
    trees: Vec<u8>,
}

impl FromStr for Forest {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut forest = Vec::new();

        for line in s.lines() {
            let mut trees: Vec<u8> = line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            forest.append(&mut trees);
        }
        
        Ok(Forest {
            row_size: s.lines().nth(0).unwrap().len(),
            col_size: s.lines().collect::<Vec<&str>>().len(),
            trees: forest,
        })
    }
}

impl Forest {
    pub fn row_size(&self) -> usize {
        self.row_size
    }

    pub fn col_size(&self) -> usize {
        self.col_size
    }

    pub fn len(&self) -> usize {
        self.trees.len()
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<&u8> {
        let index = self.row_size * row + col;
        self.trees.get(index)
    }

    pub fn viewable(&self, row: usize, col: usize, highest_tree: &mut u8) -> bool {
        let tree: &u8 = self.get(row, col).expect("in-bounds tree");

        if tree > highest_tree {
            *highest_tree = *tree;
            return true;
        }

        false
    }

    pub fn find_best_tree(&self) -> u32 {
        let mut high_score = 0;

        for row in 0..self.row_size {
            for col in 0..self.col_size {
                let score = self.scenic_score(row, col);
                if score > high_score {
                    high_score = score;
                }
            }
        }

        high_score
    }

    fn scenic_score(&self, row: usize, col: usize) -> u32 {
        let mut north_score = 0;
        let mut east_score = 0;
        let mut south_score = 0;
        let mut west_score = 0;

        let tree = self.get(row, col).unwrap();
        for current_row in (0..row).rev() {
            if self.get(current_row, col).unwrap() < tree {
                north_score += 1;
            } else {
                north_score += 1;
                break;
            }
        }

        let tree = self.get(row, col).unwrap();
        for current_col in col+1..self.col_size {
            if self.get(row, current_col).unwrap() < tree {
                east_score += 1;
            } else {
                east_score += 1;
                break;
            }
        }

        let tree = self.get(row, col).unwrap();
        for current_row in row+1..self.row_size {
            if self.get(current_row, col).unwrap() < tree {
                south_score += 1;
            } else {
                south_score += 1;
                break;
            }
        }

        let tree = self.get(row, col).unwrap();
        for current_col in (0..col).rev() {
            if self.get(row, current_col).unwrap() < tree {
                west_score += 1;
            } else {
                west_score += 1;
                break;
            }
        }

        north_score * east_score * south_score * west_score
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = include_str!("../input/test.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(forest.row_size, 5);
        assert_eq!(forest.col_size, 5);
    }

    #[test]
    fn test_get() {
        let input = include_str!("../input/test.txt");
        let forest: Forest = input.parse().unwrap();

        assert_eq!(forest.get(0, 0), Some(&3_u8));
        assert_eq!(forest.get(4, 4), Some(&0_u8));
    }
}
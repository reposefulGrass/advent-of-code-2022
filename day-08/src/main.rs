
mod forest;
mod perspective;

use forest::Forest;
use perspective::{Perspective, Direction};

fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let forest: Forest = input.parse().unwrap();
    let mut perspective = Perspective::from(&forest);

    perspective.view_from_direction(&forest, Direction::North);
    perspective.view_from_direction(&forest, Direction::East);
    perspective.view_from_direction(&forest, Direction::South);
    perspective.view_from_direction(&forest, Direction::West);

    println!("There are {} trees visible.", perspective.total_visible_trees());
}

fn part_b(input: &str) {
    let forest: Forest = input.parse().unwrap();
    let score = forest.find_best_tree();

    println!("The best tree has a scenic score of {}.", score);
}

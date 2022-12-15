
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
    let mut perspective = Perspective::new(forest.width(), forest.height());

    perspective.view_forest_from_direction(&forest, Direction::North);
    perspective.view_forest_from_direction(&forest, Direction::East);
    perspective.view_forest_from_direction(&forest, Direction::South);
    perspective.view_forest_from_direction(&forest, Direction::West);

    println!("There are {:?} visible trees.", perspective.count_visible_trees());
}

fn part_b(input: &str) {

}

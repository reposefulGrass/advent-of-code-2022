
mod rps;

use crate::rps::NaiveStrategy;
use crate::rps::SmartStrategy;

fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let strategy = NaiveStrategy::new(input);
    println!("[Part A] The total score of the naive strategy is {}.", strategy.score());
}

fn part_b(input: &str) {
    let strategy = SmartStrategy::new(input);
    println!("[Part B] The total score of the smart strategy is {}.", strategy.score());
}
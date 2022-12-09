
mod supplies;

use supplies::*;

fn main() {
    let input = include_str!("../input/test.txt");

    part_a(input);
}

fn part_a(input: &str) {
    let mut supplies = StackedSupplies::new(3);
    supplies.parse(grab_only_crates(input));
}

fn grab_only_crates(input: &str) -> &str {
    let index = input.rfind("]").unwrap();
    &input[0 .. index + 1]
}
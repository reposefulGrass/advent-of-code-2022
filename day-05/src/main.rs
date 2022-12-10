
mod supplies;

use lazy_static::lazy_static;
use regex::Regex;
use supplies::*;

lazy_static! {
    static ref COMMAND_PATTERN: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}


fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let num_crates = determine_num_of_crates(input) as u32;
    let mut supplies = StackedSupplies::new(num_crates);

    supplies.parse(grab_only_crates(input));

    for line in grab_only_commands(input).lines() {
        let (how_many, from, to) = parse_command(line);
        supplies.move_supply_ver_1(how_many, from as usize, to as usize);
    }

    println!("[Part A] The crates at the top of the stacks are {}", supplies.top_of_piles());
}

fn part_b(input: &str) {
    let num_crates = determine_num_of_crates(input) as u32;
    let mut supplies = StackedSupplies::new(num_crates);

    supplies.parse(grab_only_crates(input));

    for line in grab_only_commands(input).lines() {
        let (how_many, from, to) = parse_command(line);
        supplies.move_supply_ver_2(how_many, from as usize, to as usize);
    }

    println!("[Part B] The crates at the top of the stacks are {}", supplies.top_of_piles());
}

fn determine_num_of_crates(input: &str) -> usize {
    (input.lines().nth(0).unwrap().len() + 1) / 4
}

fn grab_only_crates(input: &str) -> &str {
    let index = input.rfind("]").unwrap();
    &input[0 .. index + 1]
}

fn grab_only_commands(input: &str) -> &str {
    let index = input.rfind("\r\n\r\n").unwrap();
    &input[index+4..]
}

fn parse_command(input: &str) -> (u32, u32, u32) {
    let captures = COMMAND_PATTERN.captures(input).unwrap();
    let how_many: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
    let from: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
    let to: u32 = captures.get(3).unwrap().as_str().parse().unwrap();
    (how_many, from - 1, to - 1)
}

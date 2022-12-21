
mod simulation;

use simulation::{Simulation, Command};

fn main() {
    let input = include_str!("../input/input.txt");

    part_a(input);
    part_b(input);
}

fn part_a(input: &str) {
    let commands: Vec<Command> = input.lines()
        .map(|line| Command::try_from(line).unwrap())
        .rev()
        .collect();

    let mut simulation = Simulation::new(commands, 2);
    
    while ! simulation.is_halted() {
        simulation.next_step();
    }

    let points_visited = simulation.points_visited();
    println!("[Part A] The simulation has determined that the rope crossed {} points.", points_visited);
}

fn part_b(input: &str) {
    let commands: Vec<Command> = input.lines()
        .map(|line| Command::try_from(line).unwrap())
        .rev()
        .collect();

    let mut simulation = Simulation::new(commands, 10);
    
    while ! simulation.is_halted() {
        simulation.next_step();
    }

    let points_visited = simulation.points_visited();
    println!("[Part B] The simulation has determined that the rope crossed {} points.", points_visited);
}

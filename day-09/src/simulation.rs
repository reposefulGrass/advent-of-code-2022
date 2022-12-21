
use std::collections::HashSet;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq)]
pub struct Command {
    direction: Direction,
    amount: u32,
}

// Point(x, y)
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point(i32, i32);

pub struct Simulation {
    step: u32,
    commands: Vec<Command>,
    visited: HashSet<Point>,
    rope: Vec<Point>,
    halted: bool,
}

impl TryFrom<&str> for Direction {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err("invalid direction"),
        }
    }
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    // Format of 's': "<direction> <amount>"
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s = s.split(" ");
        let direction: &str = s.next().ok_or("not enough tokens")?;
        let amount: u32 = s.next().ok_or("not enough_tokens")?
            .trim().parse().expect("valid amount (u32)");

        Ok(Command {
            direction: Direction::try_from(direction)?,
            amount: amount,
        })
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point(x, y)
    }
}

impl Simulation {
    pub fn new(commands: Vec<Command>, tails: usize) -> Self {
        Simulation {
            step: 0,
            commands: commands,
            visited: HashSet::new(),
            rope: vec![Point::new(0, 0); tails],
            halted: false,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn next_step(&mut self) {
        if let Some(mut command) = self.commands.pop() {
            if command.amount == 0 {
                // discard the used command; not a real simulation step.
                return;
            } else {
                command.amount -= 1;

                self.move_head(command.direction);

                for tail_id in 1..self.rope.len() {
                    self.move_tail(tail_id - 1, tail_id);
                }

                self.visited.insert(*self.rope.last().unwrap());

                self.step += 1;
                self.commands.push(command);
            }
        } else {
            self.halted = true;
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.rope.first_mut().unwrap().1 += 1,
            Direction::Right => self.rope.first_mut().unwrap().0 += 1,
            Direction::Down => self.rope.first_mut().unwrap().1 -= 1,
            Direction::Left => self.rope.first_mut().unwrap().0 -= 1,
        }
    }

    // assuming that all previous steps help the condition that the TAIL
    // is always 1-space from the HEAD, then this function holds that condition.
    fn move_tail(&mut self, head_id: usize, tail_id: usize) {
        let head: Point = *self.rope.get(head_id).unwrap();
        let tail: &mut Point = self.rope.get_mut(tail_id).unwrap();

        match (head.0 - tail.0, head.1 - tail.1) {
            // HEAD is within TAIL
            (-1, -1) | (-1, 0) | (-1, 1) |
            (0, -1)  | (0, 0)  | (0, 1)  |
            (1, -1)  | (1, 0)  | (1, 1) => {},

            // HEAD is directly above, right of, bottom of, or left of TAIL.
            (2, 0)  => { tail.0 += 1; },
            (-2, 0) => { tail.0 -= 1; },
            (0, 2)  => { tail.1 += 1; },
            (0, -2) => { tail.1 -= 1; },

            // HEAD is in right upmost corner
            (2, 1) | (1, 2) | (2, 2) => { 
                tail.1 += 1; tail.0 += 1; 
            },

            // HEAD is in left upmost corner
            (-2, 1) | (-1, 2) | (-2, 2) => { 
                tail.1 += 1; tail.0 -= 1; 
            },

            // HEAD is in right bottommost corner
            (2, -1) | (1, -2) | (2, -2) => { 
                tail.1 -= 1; tail.0 += 1; 
            },

            // HEAD is in left bottommost corner
            (-2, -1) | (-1, -2) | (-2, -2) => { 
                tail.1 -= 1; tail.0 -= 1; 
            },

            (a, b) => {
                panic!("({}, {}): TAIL has been left behind by HEAD", a, b);
            }
        }
    }

    pub fn points_visited(&self) -> u32 {
        self.visited.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_from() {
        let direction = Direction::try_from("U");
        assert_eq!(direction, Ok(Direction::Up));
    }

    #[test]
    fn test_command_from() {
        let command = Command::try_from("R 4\n");
        assert_eq!(command, Ok(Command {
            direction: Direction::Right,
            amount: 4
        }));
    }
}

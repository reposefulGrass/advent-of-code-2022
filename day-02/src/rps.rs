
use crate::rps::Rps::*;
use crate::rps::GameResult::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GameResult {
    Lost,
    Draw,
    Won,
}

#[derive(Debug)]
pub struct NaiveStrategy {
    moves: Vec<(Rps, Rps)>,
}

#[derive(Debug)]
pub struct SmartStrategy {
    moves: Vec<(Rps, GameResult)>,
}

impl TryFrom<char> for Rps {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Rps, Self::Error> {
        match value {
            'A' | 'X' => Ok(Rock),
            'B' | 'Y' => Ok(Paper),
            'C' | 'Z' => Ok(Scissors),
            _ => Err("Invalid Rps type."),
        }
    }
}

impl Rps {
    fn game_result(opponent: Rps, you: Rps) -> GameResult {
        match (opponent, you) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Won,
            (Rock, Scissors) => Lost,
            (Paper, Rock) => Lost,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Won,
            (Scissors, Rock) => Won,
            (Scissors, Paper) => Lost,
            (Scissors, Scissors) => Draw,
        }
    }

    fn score_hand(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn generate_hand(&self, outcome: GameResult) -> Rps {
        match (self, outcome) {
            (Rock, Lost) => Scissors,
            (Rock, Draw) => Rock,
            (Rock, Won) => Paper,
            (Paper, Lost) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Won) => Scissors,
            (Scissors, Lost) => Paper,
            (Scissors, Draw) => Scissors,
            (Scissors, Won) => Rock,
        }
    }
}

impl TryFrom<char> for GameResult {
    type Error = &'static str;

    fn try_from(value: char) -> Result<GameResult, Self::Error> {
        match value {
            'X' => Ok(Lost),
            'Y' => Ok(Draw),
            'Z' => Ok(Won),
            _ => Err("Invalid GameResult type."),
        }
    }
}

impl GameResult {
    fn score_outcome(&self) -> u32 {
        match self {
            Lost => 0,
            Draw => 3,
            Won  => 6,
        }
    }
}

impl NaiveStrategy {
    pub fn new(input: &str) -> Self {
        let mut moves = vec![];
        for line in input.lines() {
            let opponent = Rps::try_from(line.chars().nth(0).unwrap()).unwrap();
            let you = Rps::try_from(line.chars().nth(2).unwrap()).unwrap();
            moves.push((opponent, you));
        }
        NaiveStrategy { moves: moves }
    }

    pub fn score(&self) -> u32 {
        let mut total_score = 0;

        for (opponent, you) in self.moves.iter() {
            let outcome = Rps::game_result(*opponent, *you);

            total_score += you.score_hand();
            total_score += outcome.score_outcome();
        }

        total_score
    }
}

impl SmartStrategy {
    pub fn new(input: &str) -> Self {
        let mut moves = vec![];
        for line in input.lines() {
            let opponent = Rps::try_from(line.chars().nth(0).unwrap()).unwrap();
            let result = GameResult::try_from(line.chars().nth(2).unwrap()).unwrap();
            moves.push((opponent, result));
        }
        SmartStrategy { moves: moves }
    }

    pub fn score(&self) -> u32 {
        let mut total_score = 0;

        for (opponent, result) in self.moves.iter() {
            let your_move = opponent.generate_hand(*result);

            total_score += your_move.score_hand();
            total_score += result.score_outcome();
        }

        total_score
    }
}

#[test]
fn test_try_from_trait() {
    assert_eq!(Rps::try_from('A'), Ok(Rock));
    assert_eq!(Rps::try_from('B'), Ok(Paper));
    assert_eq!(Rps::try_from('C'), Ok(Scissors));

    assert_eq!(Rps::try_from('X'), Ok(Rock));
    assert_eq!(Rps::try_from('Y'), Ok(Paper));
    assert_eq!(Rps::try_from('Z'), Ok(Scissors));

    assert_eq!(Rps::try_from('D'), Err("Invalid Rps type."));
}

#[test]
fn test_game_result() {
    assert_eq!(Rps::game_result(Rock, Paper), Won);
    assert_eq!(Rps::game_result(Rock, Rock), Draw);
    assert_eq!(Rps::game_result(Rock, Scissors), Lost);
}
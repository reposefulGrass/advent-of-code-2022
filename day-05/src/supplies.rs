
#[derive(Debug)]
pub struct StackedSupplies {
    num_piles: u32,
    piles: Vec<Vec<char>>,
}

impl StackedSupplies {
    pub fn new(num_piles: u32) -> Self {
        let mut supplies = StackedSupplies {
            num_piles,
            piles: Vec::new(),
        };

        for _ in 0..num_piles {
            supplies.piles.push(Vec::new())
        }

        supplies
    }

    ///     [D]    
    /// [N] [C]    
    /// [Z] [M] [P]
    ///
    /// Parses the above format into a list of vectors, each containing the characters within.
    pub fn parse(&mut self, input: &str) {
        for line in input.lines() {
            for pile in 0 .. self.num_piles {
                let selected_pile: &mut Vec<char> = self.piles.get_mut(pile as usize).unwrap();

                let begin_of_crate = (pile * 4) as usize;
                if let Some(supply_crate) = Self::parse_crate(&line[begin_of_crate..begin_of_crate+3]) {
                    selected_pile.push(supply_crate);
                }
            }
        }

        for pile in 0 .. self.num_piles {
            let selected_pile: &mut Vec<char> = self.piles.get_mut(pile as usize).unwrap();
            selected_pile.reverse();
        }
    }

    /// Takes a slice of a crate " [X] " and returns the character within: X.
    /// If there is no crate "    ", then return None.
    fn parse_crate(input: &str) -> Option<char> {
        let character = input.trim().chars().nth(1)?;
        if character.is_ascii_alphabetic() {
            Some(character)
        } else {
            None
        }
    }

    /// Move `how_many` crates in reverse order from `from` to `to`. 
    pub fn move_supply_ver_1(&mut self, how_many: u32, from: usize, to: usize) {
        let pile_from = self.piles.get_mut(from).unwrap();
        let mut temp = Vec::new();
        for _ in 0..how_many {
            temp.push(pile_from.pop().unwrap());
        }
        temp.reverse();

        let pile_to = self.piles.get_mut(to).unwrap();
        for _ in 0..how_many {
            pile_to.push(temp.pop().unwrap());
        }
    }

    /// Move `how_many` crates in the same order from `from` to `to`.
    pub fn move_supply_ver_2(&mut self, how_many: u32, from: usize, to: usize) {
        let pile_from = self.piles.get_mut(from).unwrap();
        let mut temp = Vec::new();
        for _ in 0..how_many {
            temp.push(pile_from.pop().unwrap());
        }

        let pile_to = self.piles.get_mut(to).unwrap();
        for _ in 0..how_many {
            pile_to.push(temp.pop().unwrap());
        }
    }

    /// Grab the top of each pile of crates.
    pub fn top_of_piles(&mut self) -> String {
        let mut output = String::from("");

        for pile in 0..self.num_piles {
            let selected_pile: &mut Vec<char> = self.piles.get_mut(pile as usize).unwrap();
            output.push_str(&selected_pile.pop().unwrap().to_string());
        }

        output
    }
}
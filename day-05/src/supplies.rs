
pub struct StackedSupplies {
    num_piles: u32,
    piles: Vec<Vec<u32>>,
}

impl StackedSupplies {
    pub fn new(num_piles: u32) -> Self {
        let mut supplies = StackedSupplies {
            num_piles,
            piles: vec![],
        };

        for _ in 0..num_piles {
            supplies.piles.append(&mut Vec::new())
        }

        supplies
    }

    pub fn parse(&mut self, input: &str) {
        for line in input.lines() {
            for pile in 0 .. num_piles {
                
            }
        }
    }
}
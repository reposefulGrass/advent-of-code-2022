
use std::str::FromStr;

#[derive(Debug)]
pub struct Forest(Vec<Vec<u8>>);

impl FromStr for Forest {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut forest = Vec::new();

        for line in s.lines() {
            let trees: Vec<u8> = line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();

            forest.push(trees);
        }

        Ok(Forest(forest))
    }
}

impl Forest {
    pub fn width(&self) -> usize {
        self.inner().get(0).unwrap().len()
    }

    pub fn height(&self) -> usize {
        self.inner().len()
    }

    pub fn inner(&self) -> &Vec<Vec<u8>> {
        &self.0
    }
}
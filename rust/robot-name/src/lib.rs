use rand::prelude::*;

const DIGITS: &'static [char; 10] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const LETTERS: &'static [char; 26] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::generate_name(),
        }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_name();
    }

    fn generate_name() -> String {
        // A robot name consists of 2 letters followed by 3 digits
        let mut name = String::new();
        name.push(*LETTERS.choose(&mut thread_rng()).unwrap());
        name.push(*LETTERS.choose(&mut thread_rng()).unwrap());
        name.push(*DIGITS.choose(&mut thread_rng()).unwrap());
        name.push(*DIGITS.choose(&mut thread_rng()).unwrap());
        name.push(*DIGITS.choose(&mut thread_rng()).unwrap());
        name
    }
}

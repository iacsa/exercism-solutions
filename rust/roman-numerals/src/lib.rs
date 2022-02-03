use std::fmt::{Display, Formatter, Result};

const DIGITS: &'static [(u32, &'static str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

pub struct Roman {
    num: u32,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut rest = self.num;

        for &(value, digit) in DIGITS {
            while value <= rest {
                rest -= value;
                write!(f, "{}", digit)?;
            }
        }
        Ok(())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman { num }
    }
}

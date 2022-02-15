use core::ops::{Add, Mul, Neg, Sub};
use num_bigint::BigInt;
use std::cmp::Ordering;
use std::str::FromStr;

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq)]
pub struct Decimal {
    digits: BigInt,
    decimals: u32,
}

impl Add<Decimal> for Decimal {
    type Output = Decimal;
    fn add(mut self, rhs: Decimal) -> Self::Output {
        if self.decimals < rhs.decimals {
            rhs + self
        } else {
            self.digits += rhs.scaled_digits(self.decimals - rhs.decimals);
            self.simplify()
        }
    }
}

impl Neg for Decimal {
    type Output = Decimal;
    fn neg(mut self) -> Self::Output {
        self.digits = self.digits.neg();
        self
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Decimal) -> Self::Output {
        self + rhs.neg()
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;
    fn mul(mut self, rhs: Decimal) -> Self::Output {
        self.digits *= rhs.digits;
        self.decimals += rhs.decimals;
        self.simplify()
    }
}

impl Ord for Decimal {
    fn cmp(&self, rhs: &Decimal) -> Ordering {
        self.scaled_digits(rhs.decimals)
            .cmp(&rhs.scaled_digits(self.decimals))
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, rhs: &Decimal) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl PartialEq for Decimal {
    fn eq(&self, rhs: &Decimal) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Self> {
        // Compute number of decimals by locating the decimal point
        let decimals = input.find('.').map(|i| input.len() - i - 1).unwrap_or(0) as u32;
        // Get rid of the decimal point (if any)
        let input = input.replacen('.', "", 1);
        let digits = BigInt::from_str(&input[..]).ok()?;
        Some(Self { digits, decimals }.simplify())
    }

    fn scaled_digits(&self, power_of_ten: u32) -> BigInt {
        &self.digits * BigInt::from(10u32).pow(power_of_ten)
    }

    /// Remove trailing zeroes after the decimal point
    fn simplify(mut self) -> Self {
        let ten = BigInt::from(10u32);
        let zero = BigInt::from(0u32);
        while self.decimals > 0 && &self.digits % &ten == zero {
            self.digits /= &ten;
            self.decimals -= 1;
        }
        self
    }
}

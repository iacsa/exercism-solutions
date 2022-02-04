/// Find the (zero-indexed) nth prime number
pub fn nth(n: u32) -> u32 {
    (2..).filter(is_prime).nth(n as usize).unwrap()
}

fn is_prime(n: &u32) -> bool {
    (2..=((*n as f64).sqrt() as u32)).all(|k| n % k != 0)
}

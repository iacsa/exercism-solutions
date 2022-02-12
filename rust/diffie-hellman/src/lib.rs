use num_bigint::BigUint;

pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    *BigUint::from(g)
        .modpow(&BigUint::from(a), &BigUint::from(p))
        .to_u64_digits()
        .get(0)
        .unwrap_or(&0)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    *BigUint::from(b_pub)
        .modpow(&BigUint::from(a), &BigUint::from(p))
        .to_u64_digits()
        .get(0)
        .unwrap_or(&0)
}

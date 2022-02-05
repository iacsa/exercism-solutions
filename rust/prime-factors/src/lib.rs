pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = vec![];
    let mut k = 2;

    // n >= 1 would be simplest
    // however, the given condition is much faster in the worst-case scenarios,
    // e.g. when n is a large prime
    while n >= k * k {
        while n % k == 0 {
            factors.push(k);
            n /= k;
        }
        k += 1;
    }

    // because we didn't reduce down to 1, there might be one factor left
    if n > 1 {
        factors.push(n);
    }

    factors
}

pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut prime = vec![true; upper_bound + 1];

    (2..upper_bound + 1)
        .filter(|&p| {
            if prime[p] {
                (p..=upper_bound / p).for_each(|k| prime[k * p] = false);
            }
            prime[p]
        })
        .collect()
}

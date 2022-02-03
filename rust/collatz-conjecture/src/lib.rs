pub fn collatz(n: u64) -> Option<u64> {
    (n > 0).then(|| n).and_then(_collatz)
}

fn _collatz(n: u64) -> Option<u64> {
    if n == 1 {
        Some(0)
    } else {
        _next_number(n).and_then(_collatz).map(|n| n + 1)
    }
}

fn _next_number(n: u64) -> Option<u64> {
    if n % 2 == 0 {
        n.checked_div(2)
    } else {
        n.checked_mul(3).and_then(|n| n.checked_add(1))
    }
}

const SOUNDS: &'static [(usize, &'static str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: usize) -> String {
    let sounds: String = SOUNDS
        .iter()
        .filter_map(|&(p, sound)| (n % p == 0).then(|| sound))
        .collect();

    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}

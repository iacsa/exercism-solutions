use std::collections::HashMap;

pub fn count(c: char, strain: &str) -> Result<usize, char> {
    nucleotide_counts(strain)?.get(&c).cloned().ok_or(c)
}

pub fn nucleotide_counts(strain: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();

    // Default values
    for c in "AGTC".chars() {
        map.insert(c, 0);
    }

    // Count occurrences
    for c in strain.chars() {
        *map.get_mut(&c).ok_or(c)? += 1
    }

    Ok(map)
}

#![allow(unused)]

use std::collections::HashMap;

fn letter_counts(text: &[&str]) -> HashMap<char, usize> {
    text.join("").chars().fold(HashMap::new(), |mut result, c| {
        *result.entry(c).or_insert(0) += 1;
        result
    })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let available_letters = letter_counts(magazine);
    let needed_letters = letter_counts(note);

    needed_letters
        .iter()
        .all(|(k, v)| available_letters.get(k).unwrap_or(&0) >= &v)
}

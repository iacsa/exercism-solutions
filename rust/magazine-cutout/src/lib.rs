#![allow(unused)]

use std::collections::HashMap;

fn word_counts<'a>(text: &'a [&str]) -> HashMap<&'a str, usize> {
    text.iter().fold(HashMap::new(), |mut result, word| {
        *result.entry(word).or_insert(0) += 1;
        result
    })
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let available_words = word_counts(magazine);
    let needed_words = word_counts(note);

    needed_words
        .iter()
        .all(|(k, v)| available_words.get(k).unwrap_or(&0) >= &v)
}

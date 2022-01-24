use std::collections::{HashMap, HashSet};

fn letter_counts(word: &str) -> HashMap<char, usize> {
    word.chars().fold(HashMap::new(), |mut result, c| {
        *result.entry(c).or_insert(0) += 1;
        result
    })
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let given_letters = letter_counts(&lower_word);

    possible_anagrams
        .iter()
        .cloned()
        .filter(|candidate| {
            let lower_candidate = candidate.to_lowercase();
            lower_candidate != lower_word && letter_counts(&lower_candidate) == given_letters
        })
        .collect()
}

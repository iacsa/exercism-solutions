use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    // split at anything that is not alphanumeric, but keep single quotes
    // at this point we don't know whether they are used as apostrophes or quotation marks
    s.split(|c: char| !(c.is_alphanumeric() || c == '\''))
        // now remove the single quotes from the ends
        // apostrophes would not be at the ends, but in the middle of the word
        .map(|s: &str| s.trim_matches('\''))
        // the splitting & trimming might leave us with empty 'words', which we don't want to count
        .filter(|w| !w.is_empty())
        .fold(HashMap::new(), |mut dict, word| {
            *dict.entry(word.to_lowercase()).or_insert(0) += 1;
            dict
        })
}

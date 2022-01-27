use rayon::prelude::*;
use std::collections::HashMap;

fn count_rayon(input: &[&str], worker_count: usize) -> Vec<[usize; 256]> {
    let mut partial_counts = vec![[0; 256]; worker_count];
    partial_counts
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, partial_count)| {
            for k in (i..input.len()).step_by(worker_count) {
                for c in input[k]
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphabetic())
                {
                    partial_count[c as usize] += 1;
                }
            }
        });
    partial_counts
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let partial_counts = count_rayon(input, worker_count);

    let mut counts = HashMap::new();
    for partial_count in partial_counts {
        for (c, v) in partial_count.iter().enumerate().filter(|(_, v)| **v > 0) {
            *counts.entry(u8::try_from(c).unwrap() as char).or_insert(0) += v;
        }
    }
    counts
}

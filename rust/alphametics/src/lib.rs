use itertools::{Itertools, Permutations};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

struct Combinations<'a> {
    letters: &'a HashSet<char>,
    permutations: Permutations<RangeInclusive<u8>>,
}

impl<'a> Combinations<'a> {
    fn new(letters: &'a HashSet<char>) -> Self {
        Self {
            letters,
            permutations: (0..=9).permutations(letters.len()),
        }
    }
}

impl<'a> Iterator for Combinations<'a> {
    type Item = HashMap<char, u8>;
    fn next(&mut self) -> Option<HashMap<char, u8>> {
        self.permutations.next().map(|permutation| {
            let mut map = HashMap::new();
            for (&v, &c) in permutation.iter().zip(self.letters.iter()) {
                map.insert(c, v);
            }
            map
        })
    }
}

pub fn solve(puzzle: &str) -> Option<HashMap<char, u8>> {
    let (addends, sum, letters, first_letters) = parse_puzzle(puzzle)?;
    for combination in Combinations::new(&letters) {
        if first_letters.iter().any(|c| combination[c] == 0) {
            continue;
        }
        if is_solution(&combination, &addends, &sum) {
            return Some(combination);
        }
    }
    None
}

fn parse_puzzle(puzzle: &str) -> Option<(Vec<Vec<char>>, Vec<char>, HashSet<char>, HashSet<char>)> {
    let mut letters: HashSet<char> = HashSet::new();
    let (addends, sum) = puzzle.split_once("==")?;
    let sum = sum.trim().chars().collect::<Vec<_>>();
    let mut first_letters: HashSet<char> = HashSet::new();
    let addends: Vec<Vec<char>> = addends
        .split('+')
        .map(|s| s.trim().chars().collect::<Vec<_>>())
        .collect();
    for &c in &sum {
        letters.insert(c);
    }
    first_letters.insert(sum[0]);
    for a in &addends {
        first_letters.insert(a[0]);
        for &c in a {
            letters.insert(c);
        }
    }
    // Reverse these for more efficient computation
    let addends = addends
        .iter()
        .map(|a| a.iter().rev().cloned().collect())
        .collect();
    let sum = sum.iter().rev().cloned().collect();
    (letters.len() <= 10 && letters.iter().all(|c| c.is_ascii_alphabetic()))
        .then(|| (addends, sum, letters, first_letters))
}

fn is_solution(map: &HashMap<char, u8>, addends: &[Vec<char>], sum: &Vec<char>) -> bool {
    // check digit by digit instead of the whole number at once -> more efficient
    let mut carry = 0u64;
    let mut idx = 0;
    while let Some(s) = sum.get(idx) {
        let addition: u64 = addends
            .iter()
            .filter_map(|a| a.get(idx).map(|c| map[c] as u64))
            .sum::<u64>()
            + carry;
        carry = addition / 10;
        let addition = addition % 10;
        if map[s] as u64 != addition {
            return false;
        }
        idx += 1;
    }
    carry == 0

    /*
    let addition: u64 = addends.iter().map(|digits| to_number(map, digits)).sum();
    let sum: u64 = to_number(map, sum);
    addition == sum
        */
}

fn to_number(map: &HashMap<char, u8>, letters: &Vec<char>) -> u64 {
    let mut result = 0;
    for c in letters {
        result = result * 10 + map[c] as u64;
    }
    result
}

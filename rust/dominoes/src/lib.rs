pub type Value = u8;
pub type Domino = (Value, Value);

pub fn chain(dominoes: &[Domino]) -> Option<Vec<Domino>> {
    // An empty chain is a correct result
    if dominoes.len() == 0 {
        return Some(vec![]);
    }

    // Make mutable copy for `build_chain` to modify
    let mut dominoes = dominoes.to_vec();

    // The chain must be circular, so it can start with any domino
    // Choose the first and keep it fixed
    let d = dominoes[0];
    build_chain(&mut dominoes[1..], d.1, d.0).map(|_| dominoes)
}

fn build_chain(dominoes: &mut [Domino], start_with: Value, end_with: Value) -> Option<()> {
    // If the chain is closed, report success
    if dominoes.len() == 0 && start_with == end_with {
        return Some(());
    }

    // Try out every domino, until we find one that fits and has a further continuation
    for i in 0..dominoes.len() {
        // Turn the domino around if it helps
        if dominoes[i].1 == start_with {
            dominoes[i] = (dominoes[i].1, dominoes[i].0);
        }

        // Try the next domino if this one doesn't fit
        if dominoes[i].0 != start_with {
            continue;
        }

        // Bring domino to the front
        dominoes.swap(0, i);

        // Recursively continue the chain with the remaining dominoes
        // and a new restriction given by the current chosen domino
        let start_with = dominoes[0].1;
        if build_chain(&mut dominoes[1..], start_with, end_with).is_some() {
            return Some(());
        }
    }

    // Backtrack if no domino was able to complete the chain
    None
}

pub fn verse(n: usize) -> String {
    let middle = match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };

    format!(
        "{} on the wall, {}.\n{}, {} on the wall.\n",
        bottles(n),
        bottles(n).to_lowercase(),
        middle,
        bottles((n + 99) % 100).to_lowercase()
    )
}

pub fn sing(n: usize, m: usize) -> String {
    (m..n + 1).rev().map(verse).collect::<Vec<_>>().join("\n")
}

fn bottles(n: usize) -> String {
    format!(
        "{1} bottle{0} of beer",
        if n == 1 { "" } else { "s" },
        if n == 0 {
            "No more".to_string()
        } else {
            n.to_string()
        }
    )
}

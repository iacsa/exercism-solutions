const NUMBERS: &'static [char] = &[' ', '1', '2', '3', '4', '5', '6', '7', '8'];

fn is_bomb(cell: char) -> bool {
    cell == '*'
}

pub fn annotate(board: &[&str]) -> Vec<String> {
    let h = board.len();
    let w = board.get(0).map(|s| s.len()).unwrap_or(0);

    // The data structure counting neigboring mines has some padding,
    // to ensure we are never out of bounds when accessing neighbors.
    let mut counts: Vec<Vec<usize>> = vec![vec![0; w + 2]; h + 2];
    for (y, row) in board.iter().enumerate() {
        for (x, _) in row.chars().enumerate().filter(|&(_, cell)| is_bomb(cell)) {
            (0..3).for_each(|dx| (0..3).for_each(|dy| counts[y + dy][x + dx] += 1));
        }
    }

    board
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, cell)| {
                    if is_bomb(cell) {
                        cell
                    } else {
                        NUMBERS[counts[y + 1][x + 1]]
                    }
                })
                .collect()
        })
        .collect()
}

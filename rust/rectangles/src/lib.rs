pub fn count(board: &[&str]) -> usize {
    let height = board.len();
    let width = board.first().map(|row| row.len()).unwrap_or(0);

    let is_str = |x: usize, y: usize, s: &str| -> bool { &board[y][x..=x] == s };
    let is_corner = |x: usize, y: usize| -> bool { is_str(x, y, "+") };
    let is_vertical = |x: usize, y: usize| -> bool { is_corner(x, y) || is_str(x, y, "|") };
    let is_horizontal = |x: usize, y: usize| -> bool { is_corner(x, y) || is_str(x, y, "-") };

    // We scan the whole board, checking each prerequisite of a rectangle as soon as possible
    (0..height)
        .flat_map(|y1| {
            (0..width)
                // Upper left corner
                .filter(move |&x| is_corner(x, y1))
                .flat_map(move |x1| {
                    // Rectangles are symmetrical, so we only need to search in one direction
                    (y1 + 1..height)
                        // Left side
                        .take_while(move |&y| is_vertical(x1, y))
                        // Lower left corner
                        .filter(move |&y| is_corner(x1, y))
                        .map(move |y2| {
                            (x1 + 1..width)
                                // Upper and lower sides
                                .take_while(|&x| is_horizontal(x, y1) && is_horizontal(x, y2))
                                // Upper and lower right corners
                                .filter(|&x| is_corner(x, y1) && is_corner(x, y2))
                                // Right side
                                .filter(|&x| (y1 + 1..y2).all(|y| is_vertical(x, y)))
                                .count()
                        })
                })
        })
        .sum()
}

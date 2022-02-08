pub struct ChessPosition {
    x: i32,
    y: i32,
}

pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<ChessPosition> {
        (0 <= x && x < 8 && 0 <= y && y < 8).then(|| ChessPosition { x: x, y: y })
    }
}

impl Queen {
    pub fn new(cp: ChessPosition) -> Queen {
        Queen { position: cp }
    }

    pub fn can_attack(&self, other: &Self) -> bool {
        let ref p1 = self.position;
        let ref p2 = other.position;

        p1.x == p2.x || p1.y == p2.y || (p1.x - p2.x).abs() == (p1.y - p2.y).abs()
    }
}

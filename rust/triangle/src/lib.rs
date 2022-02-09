pub struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

impl Triangle {
    pub fn build(sides: [u32; 3]) -> Option<Self> {
        let [a, b, c] = sides;
        let max = sides.iter().max()?;
        (a > 0 && b > 0 && c > 0 && a + b + c > 2 * max).then(|| Self { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        !self.is_equilateral() && !self.is_scalene()
    }
}

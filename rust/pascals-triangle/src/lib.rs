pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        if row_count == 0 {
            return Self { rows: vec![] };
        }

        let mut rows = vec![vec![1]];

        for i in 1..row_count {
            let mut new_row = vec![0; i + 1];
            {
                for (j, v) in rows[i - 1].iter().enumerate() {
                    new_row[j] += v;
                    new_row[j + 1] += v;
                }
            }
            rows.push(new_row);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

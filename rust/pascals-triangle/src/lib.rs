pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for row in 0..row_count {
            if row == 0 {
                rows.push(vec![1])
            } else {
                let prev_row = rows.last().unwrap();
                rows.push(
                    std::iter::once(&0)
                        .chain(prev_row.iter())
                        .zip(prev_row.iter().chain(std::iter::once(&0)))
                        .map(|(&x, &y)| x + y)
                        .collect(),
                )
            }
        }
        Self(rows)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}

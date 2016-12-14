pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    fn compute_rows(row_count: u32) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        for i in 0..(row_count as usize) {
            let next_row = match i {
                0 => vec![1],
                _ => Self::compute_next_row(&rows[i - 1])
            };

            rows.push(next_row);
        }

        rows
    }

    fn compute_next_row(prior_row: &Vec<u32>) -> Vec<u32> {
        let prior_size = prior_row.len();
        (0..(prior_size + 1)).map(|i| {
            if i > 0 && i < prior_size {
                prior_row[i - 1] + prior_row[i]
            } else {
                1
            }
        }).collect()
    }

    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { rows: Self::compute_rows(row_count) }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.iter().cloned().collect()
    }
}

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let count = self.row_count as usize;
        // Explicitly annotate Vec<Vec<u32>> to solve type inference issues
        let mut result: Vec<Vec<u32>> = Vec::with_capacity(count);

        for i in 0..count {
            let mut row = Vec::with_capacity(i + 1);
            row.push(1);

            if i > 0 {
                // Windows cleanly yields pairs of adjacent elements from the previous row
                for window in result[i - 1].windows(2) {
                    row.push(window[0] + window[1]);
                }
                row.push(1);
            }

            result.push(row);
        }

        result
    }
}
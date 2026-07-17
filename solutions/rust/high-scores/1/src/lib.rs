#[derive(Debug)]
pub struct HighScores{
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.is_empty() {return Vec::new()}
        let mut res = self.scores.clone();
        res.sort_unstable_by(|a, b| b.cmp(a));
        if self.scores.len() >= 3 {res[0..3].to_vec()}
        else {res[0..self.scores.len()].to_vec()}
    }
}

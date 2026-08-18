pub struct RailFence{rails :usize}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        if self.rails <= 1 {
            return text.to_string();
        }
        let mut fence: Vec<Vec<char>> = vec![Vec::new(); self.rails];
        for (c, rail) in text.chars().zip(self.zig_zag_indices()) {
            fence[rail].push(c);
        }
        fence.into_iter().flatten().collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        if self.rails <= 1 {
            return cipher.to_string();
        }

        let len = cipher.len();

        let rails_idx: Vec<usize> = self.zig_zag_indices().take(len).collect();
        let mut positions: Vec<usize> = (0..len).collect();
        positions.sort_by_key(|&i| rails_idx[i]);
        let mut result = vec![' '; len];
        for (cipher_char, original_idx) in cipher.chars().zip(positions) {
            result[original_idx] = cipher_char;
        }

        result.into_iter().collect()
    }
    
    fn zig_zag_indices(&self) -> impl Iterator<Item = usize> {
        let rails = self.rails;
        (0..rails - 1).chain((1..rails).rev()).cycle()
    }
}



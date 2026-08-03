#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<u16>,
    roll_again: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::new(),
            roll_again: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        use Error::*;
        if (self.roll_again && pins + self.frames.last().unwrap() > 10) || pins > 10 {
            Err(NotEnoughPinsLeft)
        } else if self.score().is_some() {
            Err(GameComplete)
        } else {
            self.frames.push(pins);
            self.roll_again = !self.roll_again && pins != 10;
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut cr = 0;
        let mut sum = 0;
        for _ in 1..=10 {
            sum += *self.frames.get(cr)? + *self.frames.get(cr + 1)?;
            if self.frames[cr] + self.frames[cr + 1] >= 10 {
                sum += *self.frames.get(cr + 2)?;
            }
            cr += if self.frames[cr] == 10 { 1 } else { 2 };
        }
        Some(sum)
    }
}

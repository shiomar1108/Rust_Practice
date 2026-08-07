#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32,i32),
    face: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
           position: (x,y),
           face: d
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.face = match self.face {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    };
    self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.face = match self.face {
        Direction::North => Direction::West,
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
    };
    self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.face {
            Direction::North => self.position.1 +=1,
            Direction::East => self.position.0 +=1,
            Direction::South => self.position.1 -=1,
            Direction::West => self.position.0 -=1,
        };
    self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'L' => self.turn_left(),
                'A' => self.advance(),
                'R' => self.turn_right(),
                _ => todo!(),
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.face
    }
}

// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction = self.direction.turn_right();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction = self.direction.turn_left();
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::West => self.x -= 1,
            Direction::South => self.y -= 1,
            Direction::East => self.x += 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            match c {
                'L' => self = self.turn_left(),
                'R' => self = self.turn_right(),
                'A' => self = self.advance(),
                _ => {}
            }
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

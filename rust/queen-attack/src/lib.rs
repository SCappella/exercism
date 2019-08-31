const BOARD_SIZE: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if 0 <= rank && rank < BOARD_SIZE && 0 <= file && file < BOARD_SIZE {
            Some(Self { rank, file })
        } else {
            None
        }
    }

    /// Returns a number uniquely representing the bottom left to top right diagonal
    fn diag_1(&self) -> i32 {
        self.rank - self.file
    }

    /// Returns a number uniquely representing the top left to bottom right diagonal
    fn diag_2(&self) -> i32 {
        self.rank + self.file
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0.rank == other.0.rank
            || self.0.file == other.0.file
            || self.0.diag_1() == other.0.diag_1()
            || self.0.diag_2() == other.0.diag_2()
    }
}

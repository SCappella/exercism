const PINS: u16 = 10;
const NORMAL_FRAMES: u8 = 9;
const MAX_THROWS_NORMAL: u8 = 2;
const STARTING_THROWS_LAST: u8 = 2;
const MAX_THROWS_LAST: u8 = 3;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum GameState {
    // frame number and the number of throws made in this frame
    NormalFrame(u8, u8),
    // number of throws made and the maximum number of throws (usually 2 or 3).
    LastFrame(u8, u8),
    GameComplete,
}

pub struct BowlingGame {
    // The score so far
    score: u16,
    // the frame numbers and state
    state: GameState,
    // strikes and spares. Stored as the number of rolls left
    active_bonuses: Vec<u8>,
    // pins left standing
    pins: u16,
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self {
            score: 0,
            state: GameState::NormalFrame(1, 0),
            active_bonuses: Vec::new(),
            pins: PINS,
        }
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > self.pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        match self.state {
            GameState::NormalFrame(frame, throws) => self.normal_frame(frame, throws, pins),
            GameState::LastFrame(throws, max_throws) => self.last_frame(throws, max_throws, pins),
            GameState::GameComplete => Err(Error::GameComplete),
        }
    }

    fn normal_frame(&mut self, frame: u8, mut throws: u8, pins: u16) -> Result<(), Error> {
        // set the new score
        self.score += pins * (1 + self.active_bonuses.len() as u16);

        // decrement active bonuses
        self.active_bonuses.iter_mut().for_each(|b| *b -= 1);
        self.active_bonuses.retain(|&b| b > 0);

        self.pins -= pins;
        if self.pins == 0 {
            // if you finish in 1 throws, it's a strike (two bonus turns)
            // if you finish in 2 throws, it's a spare (one bonus turn)
            self.active_bonuses.push(MAX_THROWS_NORMAL - throws);
            //since we finished the frame, set throws to max
            throws = MAX_THROWS_NORMAL;
        } else {
            throws += 1;
        }

        if throws == MAX_THROWS_NORMAL {
            // advance to the next frame
            if frame == NORMAL_FRAMES {
                // advance to the last frame
                self.state = GameState::LastFrame(0, STARTING_THROWS_LAST);
            } else {
                self.state = GameState::NormalFrame(frame + 1, 0);
            }

            self.pins = PINS;
        } else {
            self.state = GameState::NormalFrame(frame, throws)
        }

        Ok(())
    }

    fn last_frame(&mut self, mut throws: u8, mut max_throws: u8, pins: u16) -> Result<(), Error> {
        // set the new score
        self.score += pins * (1 + self.active_bonuses.len() as u16);

        // decrement active bonuses
        self.active_bonuses.iter_mut().for_each(|b| *b -= 1);
        self.active_bonuses.retain(|&b| b > 0);

        self.pins -= pins;
        if self.pins == 0 {
            // add the bonus frame if it hasn't been added already
            max_throws = MAX_THROWS_LAST;
            // reset the pins for another potential throw
            self.pins = PINS;
        }

        throws += 1;

        if throws == max_throws {
            // we're done
            self.state = GameState::GameComplete;
        } else {
            self.state = GameState::LastFrame(throws, max_throws);
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if let GameState::GameComplete = self.state {
            Some(self.score)
        } else {
            None
        }
    }
}

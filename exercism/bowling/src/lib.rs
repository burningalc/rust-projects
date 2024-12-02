use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    frames: [Option<u16>; 22],
    round: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame::default()
    }

    pub fn count_fill_balls(&self) -> Option<usize> {
        // check if regular balls are played
        if self.round < 20 {
            return None;
        }

        // check for the last frame
        match (self.frames[18], self.frames[19]) {
            (Some(10), None) => Some(2), // strike
            (Some(n), Some(m)) => {
                if n + m == 10 {
                    Some(1) // spare
                } else {
                    Some(0)
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.round == 20 + self.count_fill_balls().unwrap_or(0) {
            return Err(Error::GameComplete);
        }

        // check if this is the 1st/2nd ball of the frame
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.round % 2 == 1
            && self.frames[self.round - 1].unwrap() != 10
            && self.frames[self.round - 1].unwrap() + pins > 10
        {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.frames[self.round] = Some(pins);

        // check if is a strike
        self.round += if self.round < 19 && pins == 10 && self.round % 2 == 0 {
            2
        } else {
            1
        };

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        // check if all 10 frames plus fill balls are finished
        if self.round != 20 + self.count_fill_balls().unwrap_or(0) {
            return None;
        }

        // count the score
        let mut score = 0;
        let mut multiplier_factor: VecDeque<u16> = VecDeque::new();
        multiplier_factor.push_back(1);
        multiplier_factor.push_back(1);

        for frame in self.frames.chunks(2) {
            match (frame[0], frame[1]) {
                (Some(n), None) => {
                    score += n * multiplier_factor.pop_front()?;
                    multiplier_factor.push_back(1);
                    // if strike, add 1 to 2 multiplier factors
                    multiplier_factor[0] += 1;
                    multiplier_factor[1] += 1;
                }
                (Some(n), Some(m)) => {
                    score +=
                        n * multiplier_factor.pop_front()? + m * multiplier_factor.pop_front()?;
                    multiplier_factor.push_back(1);
                    multiplier_factor.push_back(1);

                    // if spare, add 1 to 1 multiplier factor
                    if n + m == 10 {
                        multiplier_factor[0] += 1;
                    }
                }
                (None, None) => (),
                _ => unreachable!(),
            }
        }

        // deduct the score of the fill balls
        score -= self.frames[20].unwrap_or(0) + self.frames[21].unwrap_or(0);
        Some(score)
    }
}

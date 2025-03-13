const IS_EMPTY: u16 = 17;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<(u16, u16)>,
    actual_frame_num: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![(IS_EMPTY, IS_EMPTY)],
            actual_frame_num: 1,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }
        if self.is_game_complete() { return Err(Error::GameComplete) }

        let (roll_1, roll_2) = self.frames.last().unwrap();

        match self.actual_frame_num {
            11 => {
                let (last_roll_1, last_roll_2) = self.frames[self.actual_frame_num - 2];

                if Self::is_strike((last_roll_1, last_roll_2)) {
                    return match (roll_1, roll_2) {
                        (&IS_EMPTY, _) => Ok(self.frames[self.actual_frame_num - 1].0 = pins),
                        (_, &IS_EMPTY) => {
                            if roll_1 < &10 && pins > (10 - roll_1) { return Err(Error::NotEnoughPinsLeft) }
                            Ok(self.frames[self.actual_frame_num - 1].1 = pins)
                        },
                        (_, _) => Err(Error::GameComplete),
                    }
                }else if Self::is_spare((last_roll_1, last_roll_2)) {
                    return match roll_1 {
                        &IS_EMPTY => Ok(self.frames[self.actual_frame_num - 1].0 = pins),
                        _ => Err(Error::GameComplete),
                    }
                }
                Err(Error::GameComplete)
            },
            _ => {
                if Self::is_strike((*roll_1, *roll_2)) || !Self::is_frame_complete((*roll_1, *roll_2)) {
                    self.frames.push((IS_EMPTY, IS_EMPTY));
                    self.actual_frame_num = self.frames.len();
                    return self.roll(pins)
                }

                match (roll_1, roll_2) {
                    (&IS_EMPTY, _) => Ok(self.frames[self.actual_frame_num - 1].0 = pins),
                    (_, &IS_EMPTY) => {
                        if roll_1 < &10 && pins > (10 - roll_1) { return Err(Error::NotEnoughPinsLeft) }
                        Ok(self.frames[self.actual_frame_num - 1].1 = pins)
                    },
                    (_, _) => Ok(()),
                }
            },
        }
    }

    pub fn score(&self) -> Option<u16> {
        let (roll_1, roll_2) = self.frames.last().unwrap();
        match self.actual_frame_num {
            10 => {
                if Self::is_spare((*roll_1, *roll_2)) ||
                    Self::is_strike((*roll_1, *roll_2)) ||
                    (roll_1 == &IS_EMPTY || roll_2 == &IS_EMPTY) { return None } },
            11 => {
                let (last_roll_1, last_roll_2) = self.frames[self.actual_frame_num - 2];
                if Self::is_strike((last_roll_1, last_roll_2)) {
                    if roll_1 == &IS_EMPTY || roll_2 == &IS_EMPTY { return None }
                } else if Self::is_spare((last_roll_1, last_roll_2)) && roll_1 == &IS_EMPTY { return None }
            },
            _ => return None,
        }

        let normalized_frames: Vec<(u16, u16)> = Self::normalize_frames(self.frames.clone());
        let mut score_vec: Vec<u16> = vec![];

        for frame_idx in 0..normalized_frames.len() {
            let mut tmp_score = 0u16;
            match frame_idx {
                10 => (),
                _ => {
                    if Self::is_strike(normalized_frames[frame_idx]) {
                        let (actual_frame, next_frame, next_next_frame) = match frame_idx {
                            9 => (normalized_frames[frame_idx], normalized_frames[frame_idx + 1], (normalized_frames[frame_idx + 1].1, IS_EMPTY)),
                            _ => (normalized_frames[frame_idx], normalized_frames[frame_idx + 1], normalized_frames[frame_idx + 2]),
                        };
                        tmp_score += Self::calculate_strike_score(actual_frame,
                                                                  next_frame,
                                                                  next_next_frame);
                    }else if Self::is_spare(normalized_frames[frame_idx]) {
                        tmp_score += Self::calculate_spare_score(normalized_frames[frame_idx],
                                                                 normalized_frames[frame_idx + 1]);
                    }else {
                        tmp_score += Self::calculate_normal_frame_score(normalized_frames[frame_idx]);
                    }
                }
            }
            score_vec.push(tmp_score);
        }

        Some(score_vec.iter().sum())
    }

    fn normalize_frames(frames_vec: Vec<(u16, u16)>) -> Vec<(u16, u16)> {
        frames_vec.into_iter()
            .map(|tuple| {
                match tuple {
                    (17, _) => (0, tuple.1),
                    (_, 17) => (tuple.0, 0),
                    (_, _) => tuple
                }})
            .collect()
    }

    fn is_game_complete(&self) -> bool {
        match self.actual_frame_num {
            10 => (self.frames[self.actual_frame_num - 1].0 + self.frames[self.actual_frame_num - 1].1) < 10,
            11 => {
                let tenth_frame = self.frames[self.actual_frame_num - 2];
                if Self::is_strike(tenth_frame) {
                    return (self.frames[self.actual_frame_num - 1].0 <= 10) &&
                        (self.frames[self.actual_frame_num - 1].1 <= 10)
                }
                self.frames[self.actual_frame_num - 1].0 <= 10
            },
            _ => false,
        }
    }

    fn is_frame_complete(frame: (u16, u16)) -> bool {
        (frame.0 == IS_EMPTY) || (frame.1 == IS_EMPTY)
    }

    fn is_spare(frame: (u16, u16)) -> bool {
        (frame.0 + frame.1) == 10
    }

    fn is_strike(frame: (u16, u16)) -> bool {
        (frame.0 == 10) || (frame.1 == 10)
    }

    fn calculate_spare_score(actual_frame: (u16, u16), next_frame: (u16, u16)) -> u16 {
        actual_frame.0 + actual_frame.1 + next_frame.0
    }

    pub fn calculate_strike_score(actual_frame: (u16, u16), next_frame: (u16, u16), next_next_frame: (u16, u16)) -> u16 {
        let actual_frame_sum = actual_frame.0 + actual_frame.1;
        if next_frame == (IS_EMPTY, IS_EMPTY) {
            actual_frame.0 + actual_frame.1
        }else if Self::is_strike(next_frame) {
            10 + next_frame.0 + next_next_frame.0
        } else {
            if actual_frame_sum > 10 {
                return 10 + next_frame.0 + next_frame.1
            }
            actual_frame_sum + next_frame.0 + next_frame.1
        }
    }

    fn calculate_normal_frame_score(frame: (u16, u16)) -> u16 {
        frame.0 + frame.1
    }
}

fn main() {
    // let mut game = BowlingGame::new();
    //
    // for _ in 0..12 {
    //     game.roll(10).unwrap();
    // }
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());
    //
    // let mut game = BowlingGame::new();
    // for _ in 0..9 {
    //     game.roll(0).unwrap();
    //     game.roll(0).unwrap();
    // }
    // game.roll(10).unwrap();
    // game.roll(7).unwrap();
    // game.roll(7).unwrap();
    // // game.roll(2).unwrap();
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());
    //
    // let strike = BowlingGame::calculate_strike_score((10, 0), (10, 10), (10, 10));
    // println!("{}", strike);


    // let mut game = BowlingGame::new();
    //
    // let _ = game.roll(6);
    // let _ = game.roll(4);
    //
    // for _ in 0..18 {
    //     let _ = game.roll(0);
    // }
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());

    // let mut game = BowlingGame::new();
    //
    // for _ in 0..18 {
    //     let _ = game.roll(0);
    // }
    //
    // let _ = game.roll(5);
    // let _ = game.roll(5);
    //
    // println!("{:?}", game.score());
    //
    // let _ = game.roll(7);
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());

    //let mut game = BowlingGame::new();

    //for _ in 0..18 {
    //    let _ = game.roll(0);
    //}

    //let _ = game.roll(10);
    //let _ = game.roll(10);
    //println!("{:?}", game.score());

    //let _ = game.roll(10);
    //println!("{:?}", game.score());

    /*
    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }
    game.roll(2).unwrap();
    game.roll(2).unwrap();

    println!("{:?}", game.frames);
    println!("{:?}", game.score());

     */

    // let mut game = BowlingGame::new();
    //
    // let _ = game.roll(10);
    // let _ = game.roll(10);
    // let _ = game.roll(10);
    // let _ = game.roll(5);
    // let _ = game.roll(3);
    //
    // for _ in 0..12 {
    //     let _ = game.roll(0);
    // }
    //
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());

    // let mut game = BowlingGame::new();
    //
    // for _ in 0..10 {
    //     let _ = game.roll(3);
    //     let _ = game.roll(6);
    // }
    //
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());

    // let mut game = BowlingGame::new();
    //
    // for _ in 0..18 {
    //     let _ = game.roll(0);
    // }
    //
    // let _ = game.roll(10);
    // let _ = game.roll(7);
    // let _ = game.roll(3);
    //
    // println!("{:?}", game.frames);
    // println!("{:?}", game.score());

    let mut game = BowlingGame::new();

    for _ in 0..18 {
        let _ = game.roll(0);
    }

    game.roll(10).unwrap();

    game.roll(6).unwrap();
    game.roll(10).unwrap();

    println!("{:?}", game.frames);
    println!("{:?}", game.score());
}
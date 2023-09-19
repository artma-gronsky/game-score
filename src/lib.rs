use rand::Rng;

const TIMESTAMPS_COUNT: usize = 50000;

const PROBABILITY_SCORE_CHANGED: f64 = 0.0001;

const PROBABILITY_HOME_SCORE: f64 = 0.45;

const OFFSET_MAX_STEP: i32 = 3;

const INITIAL_STAMP: Stamp = Stamp {
    offset: 0,
    score: Score { home: 0, away: 0 },
};

#[derive(Debug, Clone, Copy)]
pub struct Score {
    home: i32,
    away: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Stamp {
    offset: i32,
    score: Score,
}

fn generate_stamp(previous_value: Stamp) -> Stamp {
    let score_changed: bool = rand::thread_rng().gen_bool(PROBABILITY_SCORE_CHANGED);
    let home_score_change: bool = rand::thread_rng().gen_bool(PROBABILITY_HOME_SCORE);
    let offset_change: i32 = rand::thread_rng().gen_range(1..=OFFSET_MAX_STEP);

    Stamp {
        offset: previous_value.offset + offset_change,
        score: Score {
            home: previous_value.score.home
                + if score_changed && home_score_change {
                    1
                } else {
                    0
                },
            away: previous_value.score.away
                + if score_changed && !home_score_change {
                    1
                } else {
                    0
                },
        },
    }
}

pub fn generate_game() -> Vec<Stamp> {
    let mut stamps = vec![INITIAL_STAMP];
    let mut current_stamp = INITIAL_STAMP;

    for _ in 0..TIMESTAMPS_COUNT {
        current_stamp = generate_stamp(current_stamp);
        stamps.push(current_stamp);
    }

    stamps
}

pub fn get_score(game_stamps: &[Stamp], offset: i32) -> (i32, i32) {
    if offset < 0 {
        panic!("Game offset can't be negative number");
    }

    let mut current_score = &INITIAL_STAMP.score;

    if game_stamps.is_empty() {
        return (current_score.home, current_score.away);
    }

    for game_stamp in game_stamps {
        if game_stamp.offset <= offset {
            current_score = &game_stamp.score;
        } else {
            break;
        }
    }

    (current_score.home, current_score.away)
}

#[cfg(test)]
pub mod tests {
    use crate::{get_score, Score, Stamp};

    #[test]
    fn get_score_from_few_stamps_test() {
        // Arrange
        let stamps = vec![
            Stamp {
                offset: 1,
                score: Score { home: 0, away: 0 },
            },
            Stamp {
                offset: 3,
                score: Score { home: 1, away: 0 },
            },
            Stamp {
                offset: 6,
                score: Score { home: 1, away: 1 },
            },
            Stamp {
                offset: 7,
                score: Score { home: 1, away: 2 },
            },
            Stamp {
                offset: 9,
                score: Score { home: 2, away: 2 },
            },
        ];

        // Act
        let result1 = get_score(&stamps, 4);
        let result2 = get_score(&stamps, 3);
        let result3 = get_score(&stamps, 7);
        let result4 = get_score(&stamps, 10);

        // Assert
        assert_eq!(result1, (1, 0));
        assert_eq!(result2, (1, 0));
        assert_eq!(result3, (1, 2));
        assert_eq!(result4, (2, 2));
    }

    #[test]
    fn get_score_from_empty_stamps_test() {
        // Arrange
        let stamps: Vec<Stamp> = vec![];

        // Act
        let result = get_score(&stamps, 10);

        // Assert
        assert_eq!(result, (0, 0));
    }

    #[test]
    #[should_panic]
    fn get_score_negative_number_panic_test() {
        // Arrange
        let stamps: Vec<Stamp> = vec![];

        // Act
        let result = get_score(&stamps, -1);
    }
}

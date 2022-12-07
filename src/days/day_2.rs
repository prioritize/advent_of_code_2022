use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

struct DayTwo {
    answer: u32,
}
#[derive(Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
    Error,
}
enum GameResult {
    Win,
    Loss,
    Draw,
}
impl DayTwo {
    fn parse(filename: &str) -> u32 {
        let mut file = File::open(&filename).expect(&format!("Unable to open {}", filename));
        let mut f_string = String::new();
        file.read_to_string(&mut f_string)
            .expect(&format!("Unable to read the file into a string"));
        let games: Vec<(RPS, RPS)> = f_string
            .lines()
            .map(|x| {
                let out: Vec<RPS> = x
                    .chars()
                    .map(|c| match c {
                        'A' => RPS::Rock,
                        'B' => RPS::Paper,
                        'C' => RPS::Scissors,
                        'X' => RPS::Rock,
                        'Y' => RPS::Paper,
                        'Z' => RPS::Scissors,
                        _ => RPS::Error,
                    })
                    .filter(|x| match x {
                        RPS::Rock => true,
                        RPS::Paper => true,
                        RPS::Scissors => true,
                        RPS::Error => false,
                    })
                    .collect();
                (out[0].clone(), out[1].clone())
            })
            .collect();
        let total_score = games
            .iter()
            .map(|x| {
                match_score(x)
            })
            .fold(0, |sum, val| sum + val);

        println!("My total score is for part one is {}", total_score);

        let fixed: Vec<(RPS, RPS)> = games.iter().map(|x| {
            manipulate(x)
        }).collect();
        let fixed_score = fixed.iter().map(|x| {
            match_score(x)
        }).fold(0, |acc, v| acc + v);
        println!("My total score is for part two is {}", fixed_score);

        0
    }
}
fn match_score(game: &(RPS, RPS)) -> u32 {
    let points = match evaluate(game) {
        GameResult::Win => 6,
        GameResult::Loss => 0,
        GameResult::Draw => 3,
    };
    points + score(&game)
}
fn score(guess: &(RPS, RPS)) -> u32 {
    return match guess.1 {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
        RPS::Error => {
            panic!("This shouldn't happen")
        }
    };
}
fn evaluate(game: &(RPS, RPS)) -> GameResult {
    match game.0 {
        RPS::Rock => match game.1 {
            RPS::Rock => GameResult::Draw,
            RPS::Paper => GameResult::Win,
            RPS::Scissors => GameResult::Loss,
            RPS::Error => {
                panic!("This shouldn't happen")
            }
        },
        RPS::Paper => match game.1 {
            RPS::Rock => GameResult::Loss,
            RPS::Paper => GameResult::Draw,
            RPS::Scissors => GameResult::Win,
            RPS::Error => {
                panic!("This shouldn't happen")
            }
        },
        RPS::Scissors => match game.1 {
            RPS::Rock => GameResult::Win,
            RPS::Paper => GameResult::Loss,
            RPS::Scissors => GameResult::Draw,
            RPS::Error => {
                panic!("This shouldn't happen")
            }
        },
        RPS::Error => {
            panic!("This shouldn't happen");
        }
    }
}
fn lose(game: &(RPS, RPS)) -> RPS {
    return match game.0 {
        RPS::Rock => {
            RPS::Scissors
        }
        RPS::Paper => {
            RPS::Rock
        }
        RPS::Scissors => {
            RPS::Paper
        }
        RPS::Error => {panic!("This shouldn't happen")}
    }
}
fn win(game: &(RPS, RPS)) -> RPS {
    return match game.0 {
        RPS::Rock => {
            RPS::Paper
        }
        RPS::Paper => {
            RPS::Scissors
        }
        RPS::Scissors => {
            RPS::Rock
        }
        RPS::Error => { panic!("This shouldn't happen") }
    }
}
fn manipulate(game: &(RPS, RPS)) -> (RPS, RPS) {
    return match game.1 {
        RPS::Rock => {
        //     Lose
            (game.0.clone(), lose(game))
        }
        RPS::Paper => {
            //     Need to draw
            (game.0.clone(), game.0.clone())
        }
        RPS::Scissors => {
            //     Need to win
            (game.0.clone(), win(game))
        }
        RPS::Error => {
            panic!("This shouldn't happen");
        }
    }
}
impl Display for RPS {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            RPS::Rock => {
                write!(f, "Rock")
            }
            RPS::Paper => {
                write!(f, "Paper")
            }
            RPS::Scissors => {
                write!(f, "Scissors")
            }
            RPS::Error => {
                write!(f, "Error")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn answer_day_two() {
        DayTwo::parse("input/day_2_input.txt");
    }
}

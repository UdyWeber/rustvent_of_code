use std::fs;

#[derive(Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    pub fn from_strategy_char(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            "A" => Some(Moves::Rock),
            "B" => Some(Moves::Paper),
            "C" => Some(Moves::Scissors),
            _ => None,
        }
    }

    pub fn from_result_and_opponent_move(result: &Results, opponent_move: &Moves) -> Self {
        match (result, opponent_move) {
            (Results::Win, Moves::Scissors)
            | (Results::Draw, Moves::Rock)
            | (Results::Lose, Moves::Paper) => Moves::Rock,
            (Results::Win, Moves::Rock)
            | (Results::Draw, Moves::Paper)
            | (Results::Lose, Moves::Scissors) => Moves::Paper,
            _ => Moves::Scissors
        }
    }

    pub fn value(&self) -> i64 {
        match self {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        }
    }
}

enum Results {
    Win,
    Lose,
    Draw,
}

impl Results {
    pub fn from_strategy_char(value: &str) -> Option<Self> {
        match value.to_uppercase().as_str() {
            "X" => Some(Results::Lose),
            "Y" => Some(Results::Draw),
            "Z" => Some(Results::Win),
            _ => None,
        }

    }

    // After knowing the secret elf encryption
    pub fn from_moves(opponent_move: &Moves, response_move: &Moves) -> Self {
        match (response_move, opponent_move) {
            (Moves::Paper, Moves::Rock)
            | (Moves::Rock, Moves::Scissors)
            | (Moves::Scissors, Moves::Paper) => Results::Win,
            (Moves::Paper, Moves::Paper)
            | (Moves::Rock, Moves::Rock)
            | (Moves::Scissors, Moves::Scissors) => Results::Draw,
            _ => Results::Lose
        }
    }

    pub fn value(&self) -> i64 {
        match self {
            Results::Win => 6,
            Results::Lose => 0,
            Results::Draw => 3,
        }
    }
}

pub fn day_two_resolution() -> i64 {
    let file_read = fs::read_to_string("src/day_two/my_input.txt").unwrap();

    file_read
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold( 0,  |(mut sum), (opponent_move, round_result)| {
            let opponent_move = Moves::from_strategy_char(opponent_move)
                .unwrap();

            let round_result = Results::from_strategy_char(round_result)
                .unwrap();

            let response_move = Moves::from_result_and_opponent_move(&round_result, &opponent_move);

            sum += response_move.value() + round_result.value();

            sum
        })
}

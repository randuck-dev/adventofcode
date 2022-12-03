const O_ROCK: &str = "A";
const O_PAPER: &str = "B";
const O_SCISSOR: &str = "C";

const M_ROCK: &str = "X";
const M_PAPER: &str = "Y";
const M_SCISSOR: &str = "Z";

const WIN: i32 = 6;
const DRAW: i32 = 3;
const LOOSE: i32 = 0;

const ROCK_POINTS: i32 = 1;
const PAPER_POINTS: i32 = 2;
const SCISSOR_POINTS: i32 = 3;

pub fn solve() {
    let score: i32 = include_str!("../inputs/2.txt")
        .lines()
        .map(|x| {
            let mut score_this_round = 0;
            let split_row: Vec<&str> = x.split(" ").collect();

            if split_row.len() != 2 {
                println!("unable to detect enough values to determine strategy");
                return 0;
            }

            let opponent_play = *split_row.get(0).unwrap();
            let my_play = *split_row.get(1).unwrap();

            if opponent_play == O_ROCK && my_play == M_PAPER {
                score_this_round += WIN + PAPER_POINTS
            } else if opponent_play == O_PAPER && my_play == M_SCISSOR {
                score_this_round += WIN + SCISSOR_POINTS
            } else if opponent_play == O_SCISSOR && my_play == M_ROCK {
                score_this_round += WIN + ROCK_POINTS
            } else if opponent_play == O_SCISSOR && my_play == M_SCISSOR {
                score_this_round += DRAW + SCISSOR_POINTS
            } else if opponent_play == O_ROCK && my_play == M_ROCK {
                score_this_round += DRAW + ROCK_POINTS
            } else if opponent_play == O_PAPER && my_play == M_PAPER {
                score_this_round += DRAW + PAPER_POINTS
            } else if opponent_play == O_PAPER && my_play == M_ROCK {
                score_this_round += LOOSE + ROCK_POINTS
            } else if opponent_play == O_ROCK && my_play == M_SCISSOR {
                score_this_round += LOOSE + SCISSOR_POINTS
            } else if opponent_play == O_SCISSOR && my_play == M_PAPER {
                score_this_round += LOOSE + PAPER_POINTS
            }

            score_this_round
        })
        .sum();

    println!("Final score is: {}", score)

    // COLUMN 1: A -> ROCK, B -> PAPER, C -> SCISSORS
    // COLUMN 2: X -> ROCK, Y -> PAPER, Z -> SCISSORS
}

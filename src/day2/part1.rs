use std::{fs::File, io::Read};

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

pub fn solve(base_path: &str) {
    let owned_path = base_path.to_owned();
    let path = owned_path + "/2.txt";

    let mut file = File::open(path).expect("file not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("error while reading file");

    let rows = data.split("\n");

    let mut my_score = 0;

    for row in rows {
        let mut score_this_round = 0;
        let split_row: Vec<&str> = row.split(" ").collect();

        if split_row.len() != 2 {
            println!("unable to detect enough values to determine strategy");
            continue;
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

        my_score += score_this_round
    }

    println!("Final score is: {}", my_score)

    // COLUMN 1: A -> ROCK, B -> PAPER, C -> SCISSORS
    // COLUMN 2: X -> ROCK, Y -> PAPER, Z -> SCISSORS
}
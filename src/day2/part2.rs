use std::{fs::File, io::Read};

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
        let score = match row {
            "A Y" => DRAW + ROCK_POINTS,
            "A X" => LOOSE + SCISSOR_POINTS,
            "A Z" => WIN + PAPER_POINTS,
            "B Y" => DRAW + PAPER_POINTS,
            "B X" => LOOSE + ROCK_POINTS,
            "B Z" => WIN + SCISSOR_POINTS,
            "C Y" => DRAW + SCISSOR_POINTS,
            "C X" => LOOSE + PAPER_POINTS,
            "C Z" => WIN + ROCK_POINTS,
            _ => panic!("Unable to match sequence"),
        };

        my_score += score
    }

    println!("Final score is: {}", my_score)
}

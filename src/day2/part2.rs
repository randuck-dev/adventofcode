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
            return match x {
                "A Y" => DRAW + ROCK_POINTS,
                "A X" => LOOSE + SCISSOR_POINTS,
                "A Z" => WIN + PAPER_POINTS,
                "B Y" => DRAW + PAPER_POINTS,
                "B X" => LOOSE + ROCK_POINTS,
                "B Z" => WIN + SCISSOR_POINTS,
                "C Y" => DRAW + SCISSOR_POINTS,
                "C X" => LOOSE + PAPER_POINTS,
                "C Z" => WIN + ROCK_POINTS,
                _ => unreachable!("Unable to match sequence"),
            };
        })
        .sum();

    println!("Final score is: {}", score)
}

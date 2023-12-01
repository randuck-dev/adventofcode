mod part1;
mod part2;

use adventofcode::Solvable;

pub struct Day2 {}

impl Solvable for Day2 {
    fn solve(&self) {
        println!("---- DAY 2 ----");
        part1::solve();
        part2::solve();
    }
}

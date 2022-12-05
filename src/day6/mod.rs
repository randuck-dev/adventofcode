use adventofcode::Solvable;

mod part1;
mod part2;

pub struct Day6 {}

impl Solvable for Day6 {
    fn solve(&self) {
        println!("---- DAY 6 ----");
        part1::solve();
        part2::solve();
    }
}

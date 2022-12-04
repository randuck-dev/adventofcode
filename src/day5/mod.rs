use adventofcode::Solvable;

mod part1;
mod part2;

pub struct Day5 {}

impl Solvable for Day5 {
    fn solve(&self) {
        println!("---- DAY 5 ----");
        part1::solve();
        part2::solve();
    }
}

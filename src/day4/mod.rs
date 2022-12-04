use adventofcode::Solvable;

mod part1;
mod part2;

pub struct Day4 {}

impl Solvable for Day4 {
    fn solve(&self) {
        println!("---- DAY 4 ----");
        part1::solve();
        part2::solve();
    }
}

use adventofcode::Solvable;

mod part1;
mod part2;
pub struct Day9 {}

impl Solvable for Day9 {
    fn solve(&self) {
        println!("---- DAY 8 ----");
        part1::solve();
        part2::solve();
    }
}

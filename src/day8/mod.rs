use adventofcode::Solvable;

mod part1;
mod part2;
pub struct Day8 {}

impl Solvable for Day8 {
    fn solve(&self) {
        println!("---- DAY 7 ----");
        part1::solve();
        part2::solve();
    }
}

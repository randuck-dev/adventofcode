use adventofcode::Solvable;

mod part1;
mod part2;
pub struct Day7 {}

impl Solvable for Day7 {
    fn solve(&self) {
        println!("---- DAY 7 ----");
        part1::solve();
        part2::solve();
    }
}

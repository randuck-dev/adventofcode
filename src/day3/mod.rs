use adventofcode::Solvable;

mod part1;
mod part2;

pub struct Day3 {}

impl Solvable for Day3 {
    fn solve(&self) {
        println!("---- DAY 3 ----");
        part1::solve();
        part2::solve();
    }
}

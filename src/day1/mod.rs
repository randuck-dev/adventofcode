mod part1;
mod part2;

pub struct Day1 {}

impl Day1 {
    pub fn solve(&self, path: &str) {
        println!("---- DAY 1 ----");
        part1::solve(path);
        part2::solve(path);
    }
}

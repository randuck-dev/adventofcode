mod part1;
mod part2;
pub struct Day2 {}

impl Day2 {
    pub fn solve(&self, path: &str) {
        println!("---- DAY 2 ----");
        part1::solve(path);
        part2::solve();
    }
}

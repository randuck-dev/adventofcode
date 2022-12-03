use adventofcode::Solvable;

mod day1;
mod day2;
mod day3;

fn main() {
    let day1: day1::Day1 = day1::Day1 {};
    let day2: day2::Day2 = day2::Day2 {};
    let day3: day3::Day3 = day3::Day3 {};

    let base_path = "/Users/raphaelneumann/projects/adventofcode/src/inputs";
    day1.solve(base_path);
    day2.solve();

    day3.solve();
}

use adventofcode::Solvable;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day1: day1::Day1 = day1::Day1 {};
    let day2: day2::Day2 = day2::Day2 {};
    let day3: day3::Day3 = day3::Day3 {};
    let day4: day4::Day4 = day4::Day4 {};
    let day5: day5::Day5 = day5::Day5 {};
    let day6: day6::Day6 = day6::Day6 {};
    let day7: day7::Day7 = day7::Day7 {};

    day1.solve();
    day2.solve();

    day3.solve();
    day4.solve();
    day5.solve();
    day6.solve();
    day7.solve();
}

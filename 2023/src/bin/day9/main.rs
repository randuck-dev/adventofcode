mod part1;
mod part2;
mod utils;

fn main() {
    let data = include_str!("./input.txt");
    match part1::solve(data) {
        Ok(i) => println!("P1: {}", i),
        Err(e) => panic!("{}", e),
    };

    match part2::solve(data) {
        Ok(i) => println!("P2: {}", i),
        Err(e) => panic!("{}", e),
    };
}

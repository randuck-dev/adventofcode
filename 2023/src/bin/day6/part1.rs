use std::collections::HashMap;

use crate::utils::{self, get_numbers};
pub fn solve(data: &str) -> Result<u32, &'static str> {
    let mut factors = vec![];
    let lines = data.lines().collect::<Vec<&str>>();

    let first = lines.first().unwrap();
    let last = lines.last().unwrap();
    let times = get_numbers(&first.split(":").collect::<Vec<&str>>()[1]);
    let distances = get_numbers(&last.split(":").collect::<Vec<&str>>()[1]);

    for i in 0..times.len() {
        let mut ways_to_win = 0;
        let time = times[i];
        let record_distance = distances[i];

        // Find all the winning combinations
        for v in 0..time {
            let btn_held = v as u64;
            if (btn_held * (time - btn_held)) > record_distance {
                ways_to_win += 1
            }
        }

        factors.push(ways_to_win)
    }
    let res = factors.iter().fold(1, |acc, &x| acc * x);

    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let res = solve(input).unwrap();

        assert_eq!(res, 288)
    }
}

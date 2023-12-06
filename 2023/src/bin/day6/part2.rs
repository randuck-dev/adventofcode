use std::{collections::HashMap, os::unix::raw::ino_t};

use crate::utils::{self, get_numbers, into_number};
pub fn solve(data: &str) -> Result<u32, &'static str> {
    let lines = data.lines().collect::<Vec<&str>>();

    let first = lines.first().unwrap();
    let last = lines.last().unwrap();
    let time = into_number(&first.split(":").collect::<Vec<&str>>()[1]);
    let record_distance = into_number(&last.split(":").collect::<Vec<&str>>()[1]);

    let mut ways_to_win = 0;

    // Find all the winning combinations
    for v in 0..time {
        let btn_held = v as u64;
        if (btn_held * (time - btn_held)) > record_distance {
            ways_to_win += 1
        }
    }

    Ok(ways_to_win)
}

#[cfg(test)]
mod tests {
    use crate::part2::solve;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let res = solve(input).unwrap();

        assert_eq!(res, 71503)
    }
}

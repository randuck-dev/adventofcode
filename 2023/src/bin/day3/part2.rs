use core::num;

const IGNORE: char = '.';
const GEAR: char = '*';

pub fn solve(data: &str) -> Result<u32, &'static str> {
    let mut sum = 0;

    let rows = data.lines().count();
    let columns = data
        .lines()
        .into_iter()
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .len();

    println!("{}x{}", rows, columns);

    let gear_count = 0;

    let mut state = vec![vec!['_'; columns]; rows];
    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            state[row][col] = c;
        }
    }

    for (row_i) in 0..rows {
        let mut number_raw = String::from("");
        let mut valid = false;

        for col in 0..columns {
            let c = state[row_i][col];
            if c.is_digit(10) {
                number_raw = format!("{}{}", number_raw, c);

                let left = if col != 0 {
                    state[row_i][col - 1]
                } else {
                    state[row_i][col]
                };

                let right = if col != columns - 1 {
                    state[row_i][col + 1]
                } else {
                    state[row_i][col]
                };

                let top = if row_i != 0 {
                    state[row_i - 1][col]
                } else {
                    state[row_i][col]
                };

                let bottom = if row_i != rows - 1 {
                    state[row_i + 1][col]
                } else {
                    state[row_i][col]
                };

                let upper_left = if row_i != 0 && col != 0 {
                    state[row_i - 1][col - 1]
                } else {
                    state[row_i][col]
                };

                let upper_right = if row_i != 0 && col != columns - 1 {
                    state[row_i - 1][col + 1]
                } else {
                    state[row_i][col]
                };

                let lower_left = if row_i != rows - 1 && col != 0 {
                    state[row_i + 1][col - 1]
                } else {
                    state[row_i][col]
                };

                let lower_right = if row_i != rows - 1 && col != columns - 1 {
                    state[row_i + 1][col + 1]
                } else {
                    state[row_i][col]
                };

                if left != IGNORE && !left.is_digit(10) {
                    valid = true;
                } else if right != IGNORE && !right.is_digit(10) {
                    valid = true;
                } else if top != IGNORE && !top.is_digit(10) {
                    valid = true;
                } else if bottom != IGNORE && !bottom.is_digit(10) {
                    valid = true;
                } else if upper_left != IGNORE && !upper_left.is_digit(10) {
                    valid = true;
                } else if upper_right != IGNORE && !upper_right.is_digit(10) {
                    valid = true;
                } else if lower_left != IGNORE && !lower_left.is_digit(10) {
                    valid = true;
                } else if lower_right != IGNORE && !lower_right.is_digit(10) {
                    valid = true;
                }
            }

            if gear_count {
                let number = number_raw.parse::<u32>().unwrap();
                sum += number;
                valid = false;
            }

            if !c.is_digit(10) {
                number_raw = String::from("");
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::part2::solve;

    #[test]
    fn test_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let res = solve(input).unwrap();

        assert_eq!(res, 467835)
    }
}

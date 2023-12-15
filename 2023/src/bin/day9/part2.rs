use itertools::Itertools;

use crate::utils::get_numbers;

pub fn solve(data: &str) -> Result<i128, &'static str> {
    let mut history = vec![];
    for line in data.lines().collect_vec() {
        let mut numbers = get_numbers::<i128>(line);

        let mut diffs = vec![numbers.clone()];

        loop {
            let mut current = vec![];
            for pair in numbers.windows(2) {
                let left = pair.first().unwrap();
                let right = pair[1];
                let diff = right - left;
                current.push(diff);
            }

            let all_zero = current.iter().all(|&x| x == 0);

            numbers = current.clone();
            diffs.push(current);

            if all_zero {
                break;
            }
        }

        // Now we have all the numbers. Lets predict the next one, bottom up
        let mut latest_prediction = 0;

        for i in diffs.iter().enumerate().map(|(x, _)| x).rev() {
            let current = &diffs[i];
            if i == 0 {
                break;
            }
            let next = &diffs[i - 1];

            let first_next = next.first().unwrap();

            let prediction = first_next - latest_prediction;

            latest_prediction = prediction;
        }

        history.push(latest_prediction);
    }

    Ok(history.iter().sum())
}

#[cfg(test)]
mod tests {
    use crate::part2::solve;

    #[test]
    fn test_example() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let res = solve(input).unwrap();

        assert_eq!(res, 2)
    }
}

pub fn solve(data: &str) -> Result<u32, &'static str> {
    let entries = data.lines();

    let mut sum = 0;
    for entry in entries {
        let res: Vec<char> = entry.chars().filter(|x| x.is_numeric()).collect();

        let first_digit = res.first().unwrap();
        let last_digit = res.last().unwrap();
        let number = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();

        sum += number;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_example() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let res = solve(input).unwrap();
        assert_eq!(res, 142)
    }
}

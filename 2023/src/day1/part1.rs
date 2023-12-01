pub fn solve() {
    let data = include_str!("../inputs/1.txt");

    let entries = data.lines();

    let mut sum = 0;
    for entry in entries {
        let res: Vec<char> = entry.chars().filter(|x| x.is_numeric()).collect();

        let first_digit = res.first().unwrap();
        let last_digit = res.last().unwrap();
        let number = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();

        sum += number;
    }

    println!("{}", sum)
}

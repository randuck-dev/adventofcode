const ONE: &str = "one";
const TWO: &str = "two";
const THREE: &str = "three";
const FOUR: &str = "four";
const FIVE: &str = "five";
const SIX: &str = "six";
const SEVEN: &str = "seven";
const EIGHT: &str = "eight";
const NINE: &str = "nine";

pub fn solve() {
    let data = include_str!("../inputs/1.txt");

    let entries = data.lines();

    let mut sum = 0;
    for entry in entries {
        let mut first_val = String::from("");
        let mut last_val = String::from("");
        for (pos, e) in entry.chars().enumerate() {
            if e.is_digit(10) {
                if first_val == "" {
                    first_val = format!("{}", e)
                }
                last_val = format!("{}", e);
                continue;
            }
            let sub = &entry[pos..];

            let mut digit = "";

            if sub.starts_with(ONE) {
                digit = "1"
            } else if sub.starts_with(TWO) {
                digit = "2"
            } else if sub.starts_with(THREE) {
                digit = "3"
            } else if sub.starts_with(FOUR) {
                digit = "4"
            } else if sub.starts_with(FIVE) {
                digit = "5"
            } else if sub.starts_with(SIX) {
                digit = "6"
            } else if sub.starts_with(SEVEN) {
                digit = "7"
            } else if sub.starts_with(EIGHT) {
                digit = "8"
            } else if sub.starts_with(NINE) {
                digit = "9"
            }

            if digit == "" {
                continue;
            }

            if first_val == "" {
                first_val = String::from(digit)
            }

            last_val = String::from(digit);
        }

        let number = format!("{}{}", first_val, last_val).parse::<i32>().unwrap();
        sum += number;
    }

    println!("{}", sum);
}

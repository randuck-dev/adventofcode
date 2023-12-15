use std::str::FromStr;

use regex::Regex;

pub fn get_numbers<T: FromStr>(data: &str) -> Vec<T> {
    data.split_whitespace()
        .filter_map(|v| v.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn into_number(data: &str) -> u64 {
    let space_pattern = Regex::new(r"\s+").unwrap();
    let result = space_pattern.replace_all(data, "");

    return result.parse::<u64>().unwrap();
}

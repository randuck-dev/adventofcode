use regex::Regex;

pub fn get_numbers(data: &str) -> Vec<u64> {
    data.split_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .collect::<Vec<u64>>()
}

pub fn into_number(data: &str) -> u64 {
    let space_pattern = Regex::new(r"\s+").unwrap();
    let result = space_pattern.replace_all(data, "");

    return result.parse::<u64>().unwrap();
}

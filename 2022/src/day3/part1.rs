const VALID_CASES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const OFFSET: usize = 1;

pub fn solve() {
    let data = include_str!("../inputs/3.txt");

    let res: usize = data
        .lines()
        .map(|x| {
            // Why does this need to be mutable?
            let mut black_list = std::collections::HashMap::new();

            let len = x.len();
            let mid = len / 2;
            // 123 - len=4
            let split_in_half = x.split_at(mid);

            let mut score = 0;

            for character in split_in_half.0.chars().enumerate() {
                // Check if the character exists in the other half
                let res = split_in_half.1.contains(character.1);
                if res && !black_list.contains_key(&character.1) {
                    black_list.insert(character.1, character.1);
                    // Get the priority
                    let character_priority =
                        VALID_CASES.chars().position(|y| y == character.1).unwrap() + OFFSET;
                    score += character_priority;
                }
            }

            return score;
        })
        .sum();

    println!("{}", res);
}

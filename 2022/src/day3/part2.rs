const VALID_CASES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const OFFSET: usize = 1;

pub fn solve() {
    let data = include_str!("../inputs/3.txt");

    let score = data
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|x| {
            for chara in VALID_CASES.chars() {
                let all_container_character = x.iter().all(|i| i.contains(chara));
                if all_container_character {
                    let res = VALID_CASES.chars().position(|c| c == chara).unwrap();
                    return res + OFFSET;
                }
            }
            return 0;
        })
        .sum::<usize>();

    println!("{:?}", score);
}

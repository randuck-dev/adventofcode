use std::collections::HashSet;

pub fn solve() {
    let data = include_str!("../inputs/6.txt");
    let mut res = 0;
    let mut i = 0;

    loop {
        if i + 13 > data.len() {
            break;
        }

        let mut set: HashSet<char> = HashSet::new();

        for j in 0..14 {
            let item = data.chars().nth(i + j).unwrap();
            set.insert(item);
        }

        if set.len() == 14 {
            res = i + 14;
            break;
        }

        i += 1;
    }

    println!("Part2: {}", res);
}

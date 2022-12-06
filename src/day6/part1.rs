use std::collections::HashSet;

pub fn solve() {
    let data = include_str!("../inputs/6.txt");

    let mut res = 0;

    let mut i = 0;

    loop {
        if i + 3 > data.len() {
            break;
        }

        let mut set: HashSet<char> = HashSet::new();

        let first = data.chars().nth(i).unwrap();
        let second = data.chars().nth(i + 1).unwrap();
        let third = data.chars().nth(i + 2).unwrap();
        let fourth = data.chars().nth(i + 3).unwrap();

        set.insert(first);
        set.insert(second);
        set.insert(third);
        set.insert(fourth);

        if set.len() == 4 {
            res = i + 4;
            break;
        }

        i += 1;
    }

    println!("Part1: {}", res);
}

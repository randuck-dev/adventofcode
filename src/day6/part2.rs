use std::collections::HashSet;

pub fn solve() {
    let data = include_str!("../inputs/6.txt");
    let mut res = 0;
    let mut i = 0;

    let inter = data.chars().collect::<Vec<char>>();
    let windowed = inter.windows(14);

    for val in windowed {
        let mut set: HashSet<char> = HashSet::new();

        for j in val {
            set.insert(*j);
        }

        if set.len() == 14 {
            res = (i + 14);
            break;
        }

        i += 1
    }

    println!("Part2: {}", res);
}

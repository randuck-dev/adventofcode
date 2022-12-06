use std::collections::HashSet;

pub fn solve() {
    let n_lookup = 14;
    let data = include_str!("../inputs/6.txt")
        .chars()
        .collect::<Vec<char>>();
    let windowed = data.windows(n_lookup);

    let mut res = 0;
    let mut i = 0;

    for val in windowed {
        let mut set: HashSet<char> = HashSet::new();

        for j in val {
            set.insert(*j);
        }

        if set.len() == n_lookup {
            res = i + n_lookup;
            break;
        }

        i += 1
    }

    println!("Part2: {}", res);
}

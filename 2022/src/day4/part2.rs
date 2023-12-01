use std::collections::HashSet;

pub fn solve() {
    let data = include_str!("../inputs/4.txt");

    let res: i32 = data
        .lines()
        .map(|x| {
            let splitted: Vec<&str> = x.split(',').collect();
            let first: Vec<&str> = splitted.get(0).unwrap().split('-').collect();
            let first_from = first.get(0).unwrap().parse::<i32>().unwrap();
            let first_to = first.get(1).unwrap().parse::<i32>().unwrap();
            let first_range: HashSet<i32> = HashSet::from_iter(first_from..=first_to);

            let second: Vec<&str> = splitted.get(1).unwrap().split('-').collect();
            let second_from = second.get(0).unwrap().parse::<i32>().unwrap();
            let second_to = second.get(1).unwrap().parse::<i32>().unwrap();
            let second_range: HashSet<i32> = HashSet::from_iter(second_from..=second_to);

            let intersected: Vec<&i32> = second_range.intersection(&first_range).collect();
            let len = intersected.len();
            if len == 0 {
                return 0;
            }

            return 1;
        })
        .sum();

    println!("Part2 :{:?}", res);
}

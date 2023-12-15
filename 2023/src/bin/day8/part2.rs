use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

pub fn solve(data: &str) -> Result<u64, &'static str> {
    let lines = data.lines().collect::<Vec<&str>>();

    let directionMap = lines[0].chars().collect_vec();

    for i in directionMap.iter() {
        print!("{}", i);
    }
    println!();

    let mut nodes = vec![];
    for (pos, line) in lines.iter().skip(2).enumerate() {
        println!("{}", line);

        let name = line.split("=").collect_vec().first().unwrap().trim();
        let rightSide = line
            .split("=")
            .collect_vec()
            .last()
            .unwrap()
            .trim()
            .replace("(", "")
            .replace(")", "");
        let leftName = rightSide.split(",").collect_vec().first().unwrap().trim();
        let rightName = rightSide.split(",").collect_vec().last().unwrap().trim();

        let left_pos = lines
            .iter()
            .skip(2)
            .position(|x| {
                let name = x.split("=").collect_vec().first().unwrap().trim();
                name == leftName
            })
            .unwrap();

        let right_pos = lines
            .iter()
            .skip(2)
            .position(|x| {
                let name = x.split("=").collect_vec().first().unwrap().trim();
                name == rightName
            })
            .unwrap();

        nodes.push(NetworkNode {
            index: pos,
            name: name.into(),
            is_z: name.ends_with("Z"),
            left: left_pos,
            right: right_pos,
        });
    }

    let mut steps_taken = 0;
    let node_positions = nodes
        .iter()
        .filter(|&x| x.name.ends_with("A"))
        .collect_vec();

    let mut cycles = vec![];
    let mut dir = 0;

    for candidate in &node_positions {
        let mut cycle = vec![];

        let mut step_count: u64 = 0;

        let mut first_z: Option<&NetworkNode> = Option::None;
        let mut current = *candidate;

        loop {
            while step_count == 0 || !current.name.ends_with("Z") {
                step_count += 1;
                current = match &directionMap[dir] {
                    'L' => &nodes[current.left],
                    'R' => &nodes[current.right],
                    _ => panic!("wat"),
                };
                if dir == directionMap.len() - 1 {
                    dir = 0;
                } else {
                    dir += 1;
                }
            }
            cycle.push(step_count);

            if first_z.is_none() {
                first_z = Option::Some(current);
                step_count = 0
            } else if current.index == first_z.unwrap().index {
                break;
            }
        }

        cycles.push(cycle);
    }

    let mut numbers = cycles.iter().map(|x| x.first().unwrap()).collect_vec();

    let mut lcm = **numbers.first().unwrap();
    numbers.remove(0);

    for n in numbers {
        lcm = lcm * *n / gcd(lcm, *n);
    }

    println!("Cycles: {:?}", cycles);
    println!("lcm: {}", lcm);
    Ok(lcm)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NetworkNode {
    index: usize,
    name: String,
    is_z: bool,
    left: usize,
    right: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let res = solve(input).unwrap();

        assert_eq!(res, 6)
    }
}

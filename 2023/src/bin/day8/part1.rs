use std::{collections::HashMap, rc::Rc};

use itertools::Itertools;

pub fn solve(data: &str) -> Result<u32, &'static str> {
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
            name: name.into(),
            left: left_pos,
            right: right_pos,
        });
    }

    let mut steps_taken = 0;
    let mut found = false;
    let mut current_node_pos = nodes.iter().position(|x| x.name == "AAA").unwrap();
    let mut current_step_pos = 0;

    loop {
        let current_node = &nodes[current_node_pos];

        let step = directionMap[current_step_pos];

        if current_node.name == "ZZZ" {
            break;
        }
        // Take the step
        current_node_pos = match step {
            'L' => current_node.left,
            'R' => current_node.right,
            _ => panic!("unable to recognize direction"),
        };

        if current_step_pos == directionMap.len() - 1 {
            current_step_pos = 0;
        } else {
            current_step_pos += 1;
        }

        if steps_taken % 1_000 == 0 {
            println!("Steps: {}", steps_taken)
        }

        steps_taken += 1;
    }

    Ok(steps_taken)
}

#[derive(Debug, Clone)]
struct NetworkNode {
    name: String,
    left: usize,
    right: usize,
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_example() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let res = solve(input).unwrap();

        assert_eq!(res, 6)
    }
}

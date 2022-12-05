use std::vec;

pub fn solve() {
    let data = include_str!("../inputs/5.txt");
    let mut split = data.split("\n\n");
    let position = split.next().unwrap();
    let moves = split.next().unwrap();
    let valid_lines: Vec<&str> = position.lines().filter(|i| i.contains("[")).collect();
    let potential_items = (valid_lines.get(0).unwrap().len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..potential_items {
        stacks.push(vec![]);
    }

    for x in valid_lines {
        if !x.contains("[") {
            panic!("Expected to see a character '['");
        }

        let mut i = 1;
        let mut current_position = 0;

        let x_as_chars: Vec<char> = x.chars().collect();
        loop {
            if i > x.len() {
                break;
            }

            let character = x_as_chars[i];

            i += 4;
            if character == ' ' {
                current_position += 1;
                continue;
            }

            let st = stacks.get_mut(current_position).unwrap();
            st.insert(0, character);
            current_position += 1;
        }
    }

    let whitespace_split: Vec<&str> = moves.split_ascii_whitespace().collect();
    let mut i = 0;

    while i < whitespace_split.len() {
        let move_val = whitespace_split
            .get(i + 1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let from = whitespace_split
            .get(i + 3)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;
        let to = whitespace_split
            .get(i + 5)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        let from_stack = stacks.get_mut(from).unwrap();

        let mut items_to_push: Vec<char> = vec![];
        for _ in 0..move_val {
            let val = from_stack.pop().unwrap();
            items_to_push.push(val);
        }

        let to_stack = stacks.get_mut(to).unwrap();
        for item in items_to_push {
            to_stack.push(item);
        }
        i += 6;
    }

    let top: String = stacks.iter().map(|i| i.last().unwrap()).collect();
    println!("Part1: {}", top);
}

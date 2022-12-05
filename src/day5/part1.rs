pub fn solve() {
    let data = include_str!("../inputs/test_5.txt");
    let mut split = data.split("\n\n");
    let position = split.next().unwrap();
    let moves = split.next().unwrap();
    let score: i32 = position
        .lines()
        .map(|x| {
            if !x.contains("[") {
                return 0;
            }
            let potential_items = (x.len() + 1) / 4;
            let line_length = x.len();

            let mut i = 1;
            let mut current_stack = 0;

            let x_as_chars: Vec<char> = x.chars().collect();
            loop {
                if i > x.len() {
                    break;
                }

                let character = x_as_chars[i];

                i += 4;

                if character == ' ' {
                    continue;
                }

                println!("{}", character);
            }

            match x {
                "    " => println!(""),
                _ => println!("WAT {}", x),
            }
            return 0;
        })
        .sum();

    println!("{}", score);
}

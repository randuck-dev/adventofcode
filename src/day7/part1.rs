use std::collections::HashSet;

pub fn solve() {
    let data = include_str!("../inputs/test_7.txt");

    let lines = data.lines();

    let mut current_dir = "";

    let mut path: Vec<&str> = vec![];

    for line in lines {
        if line.starts_with("$") {
            let cmd = get_command(line);

            match cmd {
                Commands::ChangeDir(value) => {
                    let owned = value.to_owned().as_str();
                    path.push(owned);
                }

                _ => continue,
            }

            dbg!(path);
        }
    }

    // println!("Part1: {}", res);
}

fn get_command(value: &str) -> Commands {
    let base_cmd = &value[2..4];
    let argument = &value[4..];
    let cmd = match base_cmd {
        "cd" => Commands::ChangeDir(argument.to_owned()),
        "ls" => Commands::List,

        _ => panic!(
            "Should not occur: Value: {} BaseCmd: {} Argument: {} ",
            value, base_cmd, argument
        ),
    };

    cmd
}

enum Commands {
    ChangeDir(String),
    List,
}

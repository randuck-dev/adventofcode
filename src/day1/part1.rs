use std::{fs::File, io::Read};

pub fn solve(base_path: &str) {
    let owned_path = base_path.to_owned();
    let path = owned_path + "/1.txt";

    let mut file = File::open(path).expect("file not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect("error while reading file");

    let entries = data.split("\n");

    let mut elf_calories: i32 = 0;
    let mut contenders: Vec<i32> = Vec::new();

    for entry in entries {
        if entry == "" {
            contenders.push(elf_calories);
            elf_calories = 0;
            continue;
        }
        elf_calories += entry.parse::<i32>().unwrap();
    }

    let first_element = contenders.iter().max_by_key(|x| x.abs()).unwrap();
    println!("Contenders: {}", first_element)
}

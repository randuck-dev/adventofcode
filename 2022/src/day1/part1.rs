pub fn solve() {
    let data = include_str!("../inputs/1.txt");

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

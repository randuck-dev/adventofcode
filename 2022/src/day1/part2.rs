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

    contenders.sort_by(|a, b| b.cmp(a));

    let res: i32 = contenders.iter().take(3).sum();

    println!("Res: {}", res)
}

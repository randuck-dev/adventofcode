use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let pwd = std::env::current_dir().unwrap();
    let dir_name = format!("{}/src/bin/day{}", pwd.to_str().unwrap(), args.day);

    if std::path::Path::new(&dir_name).exists() {
        println!("Directory {} already exists", dir_name);
        return;
    }

    std::fs::create_dir(&dir_name).unwrap();

    let template_files = vec!["main.txt", "part1.txt", "part2.txt"];
    for file in template_files {
        let src = format!(
            "{}/src/bin/generator/template/{}",
            pwd.to_str().unwrap(),
            file
        );
        let dst = format!("{}/{}", dir_name, file.replace("txt", "rs"));
        println!("src: {} dst: {}", src, dst);
        std::fs::copy(src, dst).unwrap();
    }

    std::fs::write(format!("{}/{}", dir_name, "input.txt"), "").unwrap();
}

const RED_BOUND: u32 = 12;
const GREEN_BOUND: u32 = 13;
const BLUE_BOUND: u32 = 14;

pub fn solve(data: &str) -> Result<u32, &'static str> {
    let mut sum = 0;
    for line in data.lines() {
        println!("{}", line);

        let init_split: Vec<&str> = line.split(":").collect();
        let game = init_split[0];

        let game_id = game.split(" ").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

        let is_valid = is_valid_game(init_split[1].split(";").collect::<Vec<&str>>());

        if !is_valid {
            continue;
        }
        sum += game_id
    }

    Ok(sum)
}

fn is_valid_game(moves: Vec<&str>) -> bool {
    for play in moves.into_iter().map(|x| x.trim()) {
        let dices = play
            .split(",")
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| get_dice(x))
            .collect::<Vec<Dice>>();

        for dice in dices {
            let valid = match dice.color {
                DiceColor::Blue => dice.count <= BLUE_BOUND,
                DiceColor::Green => dice.count <= GREEN_BOUND,
                DiceColor::Red => dice.count <= RED_BOUND,
            };

            if !valid {
                return false;
            }
        }
    }

    true
}

fn get_dice(raw_dice: &str) -> Dice {
    let s = raw_dice.trim().split(" ").collect::<Vec<&str>>();

    let count = s[0].parse::<u32>().unwrap();
    let color = match s[1] {
        "red" => DiceColor::Red,
        "blue" => DiceColor::Blue,
        "green" => DiceColor::Green,
        _ => panic!("Expected proper color"),
    };

    Dice { count, color }
}

struct Dice {
    count: u32,
    color: DiceColor,
}

enum DiceColor {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use crate::part1::solve;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let res = solve(input).unwrap();

        assert_eq!(res, 8)
    }
}

use std::collections::{HashMap, HashSet};

pub fn solve(data: &str) -> Result<u32, &'static str> {
    let mut cards = HashMap::new();

    for line in data.lines() {
        println!("{}", line);
        let split = line.split("|").collect::<Vec<&str>>();

        let left = split[0];
        let right = split[1];

        let card_id = find_card_id(left);
        let winning_numbers = find_winning_numbers(left);
        let my_numbers = get_numbers(right);
        let mut wins = 0;

        for n in my_numbers {
            if winning_numbers.contains(&n) {
                wins += 1
            }
        }

        // We must add THIS card to the stack of cards

        // This card has won
        if wins > 0 {
            // Get the number of copies that we are interested in
            let exists = cards.get(&card_id);

            let mut copies = 1;

            match exists {
                // There exists n copies
                Some(n) => {
                    // For the n copies, we need to insert all the wins
                    copies += *n;
                }
                // Entry does not exist yet
                None => {}
            }
            println!("cardId: {} copies: {} wins: {}", card_id, copies, wins);
            for i in 0..(copies) {
                // For each copy we then need to insert all the wins
                for j in 0..(wins) {
                    let card_to_add = card_id + j + 1;
                    *cards.entry(card_to_add).or_insert(0) += 1;
                }
            }
        }

        // We must always add the card itself
        *cards.entry(card_id).or_insert(0) += 1;
    }

    let res = cards
        .into_iter()
        .inspect(|(k, g)| println!("key: {} count: {}", k, g))
        .map(|(k, g)| g)
        .reduce(|acc, e| acc + e)
        .unwrap();
    Ok(res as u32)
}

fn find_card_id(input: &str) -> u32 {
    let raw = input.trim().split(":").collect::<Vec<&str>>();
    let id = raw[0].split_whitespace().last().unwrap();

    id.parse::<u32>().unwrap()
}

fn find_winning_numbers(input: &str) -> HashSet<u32> {
    let mut hs = HashSet::new();

    let raw = input.trim().split(":").collect::<Vec<&str>>();
    let numbers = get_numbers(raw[1]);

    for key in numbers {
        hs.insert(key);
    }

    hs
}

fn get_numbers(input: &str) -> Vec<u32> {
    let right = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    right
}

#[cfg(test)]
mod tests {
    use crate::part2::solve;

    #[test]
    fn test_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = solve(input).unwrap();

        assert_eq!(res, 30)
    }
}

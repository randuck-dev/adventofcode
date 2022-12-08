pub fn solve() {
    let data: Vec<&str> = include_str!("../inputs/test_8.txt").lines().collect();
    let columns = data.get(0).unwrap().len();
    let rows = data.len();
    let mut grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let mut another_grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let n_corners = 4;
    let edges_visible = ((2 * rows) + (2 * columns)) - n_corners;

    let mut current_row = 0;
    for row in data {
        let mut current_char = 0;
        for val in row.chars().into_iter().map(|c| c.to_digit(10).unwrap()) {
            grid[current_row][current_char] = val;
            another_grid[current_row][current_char] = val;

            current_char += 1;
        }

        current_row += 1;
    }

    let mut row_start = 1;
    let mut col_start = 1;

    for row in grid {
        // we are now almost at the bottom or the edge
        if row_start + 2 == rows || col_start + 2 == columns {
            break;
        }

        for col in row {
            let position = Tree_Position {
                value: another_grid[row_start][col_start],
                x: col_start,
                y: row_start,
            };
            let top = is_visible_from_top(&position, &another_grid);

            let left = is_visible_from_left(position, &another_grid);

            col_start += 1;
        }
        // Check if visible from every angle

        row_start += 1;
        col_start = 1;
    }

    let part1 = edges_visible;
    println!("Part1: {}", part1);
    // println!("Part2: {}", part2);
}

struct Tree_Position {
    value: u32,
    x: usize,
    y: usize,
}

fn is_visible_from_top(position_to_check: &Tree_Position, grid: &Vec<Vec<u32>>) -> bool {
    let mut col = position_to_check.x;

    let mut row_i = 0;

    for row in grid {
        let current = grid[row_i][col];

        if row_i == position_to_check.x && col == position_to_check.y {
            println!("Finished top checking");

            break;
        }

        if current > position_to_check.value {
            println!(
                "Position cannot be seen from top: Position: {} Overshadowing: {}",
                position_to_check.value, current
            );
            return false;
        }

        println!("Current: {} Val: {}", current, position_to_check.value);

        row_i += 1;
    }

    return true;
}

fn is_visible_from_left(position_to_check: Tree_Position, grid: &Vec<Vec<u32>>) -> bool {
    let mut col = 0;

    let row_i = position_to_check.y;

    for row in grid {
        let current = grid[row_i][col];

        if row_i == position_to_check.x && col == position_to_check.y {
            println!("Finished left checking");

            break;
        }

        if current > position_to_check.value {
            println!(
                "Position cannot be seen from left: Position: {} Overshadowing: {}",
                position_to_check.value, current
            );
            return false;
        }

        println!("Current: {} Val: {}", current, position_to_check.value);

        col += 1;
    }

    return true;
}

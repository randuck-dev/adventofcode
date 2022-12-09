use std::fmt::{self, Display};

pub fn solve() {
    let data: Vec<&str> = include_str!("../inputs/8.txt").lines().collect();
    let columns = data.get(0).unwrap().len();
    let rows = data.len();
    let mut grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let mut another_grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let n_corners = 4;
    let mut edges_visible = ((2 * rows) + (2 * columns)) - n_corners;

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
        if row_start + 1 == rows {
            break;
        }

        for col in row {
            if col_start + 1 == columns {
                break;
            }
            let position = TreePosition {
                value: another_grid[row_start][col_start],
                x: col_start,
                y: row_start,
            };

            println!("{}", position);
            let top = is_visible_from_top(&position.clone(), &another_grid);
            let left = is_visible_from_left(position.clone(), &another_grid);
            let right = is_visible_from_right(position.clone(), &another_grid);
            let bottom = is_visible_from_bottom(position.clone(), &another_grid);

            if top || left || right || bottom {
                edges_visible += 1;
            }
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

#[derive(Clone, Copy)]
struct TreePosition {
    value: u32,
    x: usize,
    y: usize,
}

impl Display for TreePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ({}, {})", self.value, self.x, self.y)
    }
}

fn is_visible_from_top(position_to_check: &TreePosition, grid: &Vec<Vec<u32>>) -> bool {
    let mut col = position_to_check.x;

    let mut row_i = 0;

    for row in grid {
        let current = grid[row_i][col];

        if row_i == position_to_check.y && col == position_to_check.x {
            break;
        }

        if current >= position_to_check.value {
            println!(
                "Position cannot be seen from top: Position: {} Overshadowing: {}",
                position_to_check, current
            );
            return false;
        }

        row_i += 1;
    }

    return true;
}

fn is_visible_from_left(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> bool {
    let mut col = 0;

    let row_i = position_to_check.y;

    for row in grid {
        let current = grid[row_i][col];

        if row_i == position_to_check.y && col == position_to_check.x {
            break;
        }

        if current >= position_to_check.value {
            println!(
                "Position cannot be seen from left: Position: {} Overshadowing: {}",
                position_to_check, current
            );
            return false;
        }

        col += 1;
    }

    return true;
}

fn is_visible_from_right(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> bool {
    let row_i = position_to_check.y;

    for i in (0..grid.len()).rev() {
        let current = grid[row_i][i];

        if row_i == position_to_check.y && i == position_to_check.x {
            break;
        }

        if current >= position_to_check.value {
            println!(
                "Position cannot be seen from right: Position: {} Overshadowing: {}",
                position_to_check, current
            );
            return false;
        }
    }

    return true;
}

fn is_visible_from_bottom(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> bool {
    let col_i = position_to_check.x;

    let row_length = grid[0].len();

    for i in (0..row_length).rev() {
        let current = grid[i][col_i];

        if col_i == position_to_check.x && i == position_to_check.y {
            break;
        }

        if current >= position_to_check.value {
            println!(
                "Position cannot be seen from bottom: Position: {} Overshadowing: {}",
                position_to_check, current
            );
            return false;
        }
    }

    return true;
}

use std::fmt::{self, Display};

pub fn solve() {
    let data: Vec<&str> = include_str!("../inputs/8.txt").lines().collect();
    let columns = data.get(0).unwrap().len();
    let rows = data.len();
    let mut grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let mut another_grid: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    let n_corners = 4;
    let mut best_score = 0;

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

            let top = look_top(&position.clone(), &another_grid);
            let left = look_left(position.clone(), &another_grid);
            let right = look_right(position.clone(), &another_grid);
            let bottom = look_bottom(position.clone(), &another_grid);

            // println!("t:{} l:{} r:{} b:{}", top, left, right, bottom);
            let score = top * left * right * bottom;

            if score > best_score {
                best_score = score;
            }
            // println!(
            //     "score for position: score: {} position: {}",
            //     score, position
            // );
            col_start += 1;
        }
        // Check if visible from every angle

        row_start += 1;
        col_start = 1;
    }

    let part2 = best_score;
    println!("Part2: {}", part2);
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

fn look_top(position_to_check: &TreePosition, grid: &Vec<Vec<u32>>) -> usize {
    let mut col_x = position_to_check.x;

    let mut scenic = 0;
    for row_y in (0..position_to_check.y).rev() {
        let current = grid[row_y][col_x];

        if row_y == position_to_check.y && col_x == position_to_check.x {
            break;
        }

        scenic += 1;

        if current >= position_to_check.value {
            return scenic;
        }
    }

    return scenic;
}

fn look_left(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> usize {
    let row_y = position_to_check.y;
    let mut scenic = 0;
    for col_x in (0..position_to_check.x).rev() {
        let current = grid[row_y][col_x];

        if col_x == position_to_check.x && row_y == position_to_check.y {
            break;
        }

        scenic += 1;

        if current >= position_to_check.value {
            return scenic;
        }
    }

    return scenic;
}

fn look_right(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> usize {
    let row_y = position_to_check.y;
    let mut scenic = 0;
    let total_cols = grid.len();
    for col_x in position_to_check.x + 1..total_cols {
        let current = grid[row_y][col_x];

        if col_x == position_to_check.x && row_y == position_to_check.y {
            break;
        }

        scenic += 1;

        if current >= position_to_check.value {
            return scenic;
        }
    }

    return scenic;
}

fn look_bottom(position_to_check: TreePosition, grid: &Vec<Vec<u32>>) -> usize {
    let col_x = position_to_check.x;
    let total_rows = grid[0].len();
    let mut scenic = 0;

    for row_y in (position_to_check.y + 1)..total_rows {
        let current = grid[row_y][col_x];

        if col_x == position_to_check.x && row_y == position_to_check.y {
            break;
        }

        scenic += 1;

        if current >= position_to_check.value {
            return scenic;
        }
    }

    return scenic;
}

advent_of_code::solution!(14);

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn count_lines_to_bottom(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row in (0..grid.len()).rev() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                count += 1;
                let mut current_row = row + 1;
                while current_row < grid.len() {
                    count += 1;
                    current_row += 1;
                }
            }
        }
    }
    count
}

pub fn move_up(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                let mut new_row = row;
                while new_row > 0 && grid[new_row - 1][col] == '.' {
                    new_row -= 1;
                }
                if new_row != row {
                    grid[new_row][col] = 'O';
                    grid[row][col] = '.';
                }
            }
        }
    }
    grid
}

pub fn move_left(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                let mut new_col = col;
                while new_col > 0 && grid[row][new_col - 1] == '.' {
                    new_col -= 1;
                }
                if new_col != col {
                    grid[row][new_col] = 'O';
                    grid[row][col] = '.';
                }
            }
        }
    }
    grid
}

pub fn move_right(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in 0..grid.len() {
        for col in (0..grid[row].len()).rev() {
            if grid[row][col] == 'O' {
                let mut new_col = col;
                while new_col < grid[row].len() - 1 && grid[row][new_col + 1] == '.' {
                    new_col += 1;
                }
                if new_col != col {
                    grid[row][new_col] = 'O';
                    grid[row][col] = '.';
                }
            }
        }
    }
    grid
}

pub fn move_down(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for row in (0..grid.len()).rev() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'O' {
                let mut new_row = row;
                while new_row < grid.len() - 1 && grid[new_row + 1][col] == '.' {
                    new_row += 1;
                }
                if new_row != row {
                    grid[new_row][col] = 'O';
                    grid[row][col] = '.';
                }
            }
        }
    }
    grid
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn calculate_checksum(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn move_four(mut grid: Vec<Vec<char>>) -> (Vec<Vec<char>>, u64) {
    grid = move_up(grid);
    grid = move_left(grid);
    grid = move_down(grid);
    grid = move_right(grid);
    let grid_string: String = grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n");
    let checksum = calculate_checksum(&grid_string);
    (grid, checksum)
}

pub fn part_one(input: &str) -> Option<u32> {
    return count_lines_to_bottom(&move_up(get_grid(input))).try_into().ok();
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = get_grid(input);
    let mut checksums = vec![];
    let mut increment = 0;
    let max_increment = 1000000000;
    while increment < max_increment {
        increment += 1;
        let check;
        (grid, check) = move_four(grid);
        if checksums.contains(&check) {
            let ecart = checksums.len() - checksums.iter().position(|&x| x == check).unwrap();
            increment += (max_increment - increment) / ecart * ecart;
            while increment < max_increment {
                increment += 1;
                (grid, _) = move_four(grid);
            }
        } else {
            checksums.push(check);
        }
    }
    return count_lines_to_bottom(&grid).try_into().ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}

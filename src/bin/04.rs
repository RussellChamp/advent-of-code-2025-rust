advent_of_code::solution!(4);
use core::iter::Iterator;

use itertools::Itertools;

#[derive(PartialEq)]
enum Item {
    Empty,
    Roll
}

const DIRECTIONS: &[(isize, isize); 8] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),          (0, 1),
    (1, -1),  (1, 0), (1, 1),
];

fn roll_count(grid: &Vec<Vec<Item>>, row_idx: usize, col_idx: usize) -> Option<usize> {
    if grid[row_idx][col_idx] != Item::Roll {
        return None;
    }

    let is_valid = |r: isize, c: isize| -> bool {
        r >= 0 && (r as usize) < grid.len() && c >= 0 && (c as usize) < grid[0].len()
    };

    let count = DIRECTIONS.iter().filter(|d| {
        let new_pos = (row_idx as isize + d.0, col_idx as isize + d.1);

        let valid = is_valid(new_pos.0, new_pos.1) && (grid[new_pos.0 as usize][new_pos.1 as usize] == Item::Roll);
        valid
    }).count();

    Some(count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = input.split('\n').filter(|l| !l.is_empty()).collect_vec();
    let row_count = rows.len() as usize;
    let col_count = rows[0].len() as usize;
    let grid = rows.into_iter().map(|r| r.chars().map(|c| match c {
        '.' => Item::Empty,
        '@' => Item::Roll,
        _ => panic!("Unknown character"),
    }).collect_vec()).collect_vec();
    // println!("Rows: {row_count}, Cols: {col_count}");

    let mut total_rolls: u64 = 0;
    for r in 0..row_count {
        for c in 0..col_count {
            if grid[r][c] != Item::Roll {
                continue;
            }

            let rolls = roll_count(&grid, r, c).unwrap();
            if rolls < 4 {
                total_rolls += 1;
                // println!("{} rolls@({}, {})", rolls, r, c);
            }
        }
    }
    Some(total_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows = input.split('\n').filter(|l| !l.is_empty()).collect_vec();
    let row_count = rows.len() as usize;
    let col_count = rows[0].len() as usize;
    let mut grid = rows.into_iter().map(|r| r.chars().map(|c| match c {
        '.' => Item::Empty,
        '@' => Item::Roll,
        _ => panic!("Unknown character"),
    }).collect_vec()).collect_vec();

    let mut total_rolls_removed = 0;
    loop {
        let mut rolls_removed = 0;
        for r in 0..row_count {
            for c in 0..col_count {
                if grid[r][c] != Item::Roll {
                    continue;
                }

                let rolls = roll_count(&grid, r, c).unwrap();
                if rolls < 4 {
                    grid[r][c] = Item::Empty;
                    rolls_removed += 1;
                    // println!("{} rolls@({}, {})", rolls, r, c);
                }
            }
        }

        total_rolls_removed += rolls_removed;
        // println!("Rolls removed this round: {} (total: {})", rolls_removed, total_rolls_removed);
        if rolls_removed == 0 {
            break;
        }
    }

    Some(total_rolls_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13)); // 2484 too high
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

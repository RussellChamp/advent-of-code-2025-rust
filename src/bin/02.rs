advent_of_code::solution!(2);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for range in input.split(',') {
        // println!("Range is: {}", range);
        let values = range.split('-').map(|v| v.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let start = values[0];
        let end = values[1];
        // println!("Range: {}-{}", start, end);
        for n in start..=end {
            let s = n.to_string();
            if s.chars().collect_vec().len() % 2 != 0 {
                continue;
            }
            let first_half = &s[0..(s.len() / 2)];
            let second_half = &s[(s.len() / 2)..s.len()];
            if first_half == second_half {
                // println!("  Number: {} ({} | {})", n, first_half, second_half);
                sum += n;
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for range in input.split(',') {
        // println!("Range is: {}", range);
        let values = range.split('-').map(|v| v.trim().parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let start = values[0];
        let end = values[1];
        // println!("Range: {}-{}", start, end);
        for n in start..=end {
            let s = n.to_string();
            let total_chars = s.chars().collect_vec().len();
            'takeBy: for take_by in 1..=(total_chars / 2) {
                if total_chars % take_by != 0 {
                    continue;
                }
            let mut idx = 0;
            while idx + 2 * take_by <= total_chars {
                let part = &s[idx..(idx + take_by)];
                let next_part = &s[(idx + take_by)..(idx + 2 * take_by)];
                if part != next_part {
                    continue 'takeBy;
                }
                idx += take_by;
            }
            // println!("  Number: {} (take by {})", n, take_by);
            sum += n;
            break; // do not continue checking other take_by values
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

advent_of_code::solution!(3);

fn get_largest_battery(battery: &[u32], start_idx: usize, end_idx: usize) -> Option<(u32, usize)> {
    let mut largest = battery[start_idx];
    let mut largest_idx = start_idx;
    for i in start_idx..=end_idx {
        if battery[i] > largest {
            largest = battery[i];
            largest_idx = i;
        }
    }
    Some((largest, largest_idx))
}

fn get_joltage(line: &str, desired_digits: u8) -> Option<u64> {
    let all_values = line.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    // determine the largest joltage by finding the largest battery in the list (excluding the last item) and then the largest battery after that
    let mut total: u64 = 0;
    let mut start_idx = 0;

    // print!("Calculating joltage for line: {} -> ", line);
    for i in 1..=desired_digits {
        let end_idx = all_values.len() - (1 + desired_digits - i) as usize;
        // print!("Finding largest between {} and {}", start_idx, end_idx);
        let (largest, largest_idx) = get_largest_battery(&all_values, start_idx, end_idx).unwrap();
        // println!(" -> found {}", largest);
        total = total * 10 + largest as u64;
        start_idx = largest_idx + 1;
    }
    Some(total)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.split('\n').filter(|l| !l.is_empty()) {
        total += get_joltage(line, 2).unwrap();
    }

    println!("Total: {}", total);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.split('\n').filter(|l| !l.is_empty()) {
        total += get_joltage(line, 12).unwrap();
    }

    println!("Total: {}", total);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

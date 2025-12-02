advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let dir_regex = Regex::new(r"(.)(\d+)").unwrap();

    let (hits, _total) = dir_regex.captures_iter(input).fold((0u32, 50i32), |(hits, total), cap| {
        let (_, [direction, value_str]) = cap.extract();
        let value = value_str.parse::<i32>().unwrap();

        let new_total = (total + 100 + if direction == "L" { -1i32 } else { 1 } * value) % 100;
        let new_hits = if new_total == 0 { hits + 1 } else { hits };

        // println!("{} + {} {} = {}", total, direction, value, new_total);

        (new_hits, new_total)
    });

    Some(hits as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
        let dir_regex = Regex::new(r"(.)(\d+)").unwrap();

    let (hits, _total) = dir_regex.captures_iter(input).fold((0u32, 50i32), |(hits, total), cap| {
        let (_, [direction, value_str]) = cap.extract();
        let value: i32 = value_str.parse::<i32>().unwrap();

        let new_total_unmod = total + if direction == "L" { -1i32 } else { 1 } * value;
        let new_total = (new_total_unmod % 100 + 100) % 100;
        let clicks_past_zero = (new_total_unmod / 100).abs() as u32 + if total != 0 && new_total_unmod <= 0 { 1 } else { 0 };

        // println!("{} + {} {} = {} ({} total clicks, {} new)", total, direction, value, new_total, hits + clicks_past_zero, clicks_past_zero);

        (hits + clicks_past_zero, new_total)
    });

    Some(hits as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

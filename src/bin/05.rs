advent_of_code::solution!(5);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u64> {
    let lists = input.split("\n\n").collect_vec();
    let ranges = lists[0]
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            // println!("Parsed range: {}-{}", start, end);
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect_vec();
    let ids = lists[1].lines().map(|line| line.parse::<u64>().unwrap()).collect_vec();

    // println!("{ranges:?}");
    // println!("{ids:?}");
    let mut fresh_item_count = 0;

    for id in ids {
        for (start, end) in &ranges {
            if id >= *start && id <= *end {
                // println!("ID {id} is in range {start}-{end}");
                fresh_item_count += 1;
                break;
            }
        }
    }

    Some(fresh_item_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lists = input.split("\n\n").collect_vec();
    let mut ranges = lists[0]
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            // println!("Parsed range: {}-{}", start, end);
            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect_vec();
    ranges.sort_by(|a, b| if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        });
    // println!("{ranges:?}");

    let mut i = 0;
    while i < ranges.len() - 1 {
        let (start_a, end_a) = ranges[i];
        let (start_b, end_b) = ranges[i + 1];
        if end_a >= start_b {
            let new_end = end_a.max(end_b);
            ranges[i] = (start_a, new_end);
            ranges.remove(i + 1);
            // println!("Merged to: {:?}", ranges[i]);
        }
        else {
            i += 1;
        }
    }
    // println!("{ranges:?}");

    let total_count = ranges.iter().map(|(start, end)| end - start + 1).sum::<u64>();
    // sort and merge ranges

    Some(total_count)
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
        assert_eq!(result, Some(14));
    }
}

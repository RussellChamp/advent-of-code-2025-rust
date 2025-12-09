advent_of_code::solution!(6);
use itertools::{Itertools, izip};
use regex::Regex;
use regex_split::RegexSplit;

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().filter(|l| !l.is_empty()).rev().collect_vec();
    let commands = lines[0].split_ascii_whitespace().map(|c| c.chars().nth(0).unwrap()).collect_vec();
    // println!("{commands:?}");
    let values = &lines[1..];
    let starting_values = commands.iter().map(|c| match c {
        '+' => 0u64,
        '*' => 1u64,
        _ => 0u64,
    }).collect_vec();
    let subtotals = values.iter().fold(starting_values, |totals, row| {
        let row_values = row.split_ascii_whitespace().map(|v| v.parse::<u64>().unwrap()).collect_vec();
        // println!("{row_values:?}");
        izip!(totals, row_values, commands.clone()).map(|(total, row_value, command)| {
            match command {
                '+' => total + row_value,
                '*' => total * row_value,
                _ => total,
            }
        }).collect_vec()
        // totals.iter().zip(row_values)
    });
    // println!("{subtotals:?}");

    Some(subtotals.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
    // println!("{lines:?}");
    let re = Regex::new(r"(.\s+)").unwrap();
    let commands_raw = re.split_inclusive(lines.last().unwrap()).filter(|c| !c.is_empty()).collect_vec();
    let commands = commands_raw.iter().enumerate().map(|(i, c)| {
        // println!("'{c}'");
        // all columns except the last one will have an extra space that is used to split that column from the next one. the last column does not have that space.
        if i == commands_raw.len() -1 { (c.chars().nth(0).unwrap(), c.len()) } else { (c.chars().nth(0).unwrap(), c.len() - 1) }
        // (c.chars().nth(0).unwrap(), c.len())
    }).collect_vec();
    // println!("{commands:?}");
    let values = &lines[..lines.len()-1];
    let mut grand_total = 0;
    // iterate through each column, reading the rows multiple time. this will not be optimal but i think it will work
    let mut cur_offset = 0;
    for (command, offsets) in commands {
        // values.iter().for_each(|item| println!("{item:?}"));
        let mut column_values: Vec<u64> = vec![0; offsets];
        // the lines ["123", " 45", "  6"] should be read right to left and transformed into [356, 24, 1]
        for row in values {
            // println!("reading from {} to {}", cur_offset, cur_offset+offsets - 1);
            for vidx in cur_offset..cur_offset+offsets {
                let c = row.to_string().chars().nth(vidx).unwrap();
                // println!("{c}");
                let cidx = vidx - cur_offset;
                if c == ' ' { continue }
                let cval = c.to_digit(10).unwrap() as u64;
                column_values[cidx] = if column_values[cidx] == 0 { cval } else {
                    column_values[cidx] * 10 + cval
                };
                // println!("{column_values:?}");
            }
        }
        cur_offset = cur_offset + offsets + 1;

        let subtotal: u64 = if command == '*' { column_values.iter().product() } else { column_values.iter().sum() };
        // print!("command: {}", command);
        // print!(" - values: {:?}", column_values);
        // print!(" - subtotal: {}", subtotal);
        // println!();
        grand_total += subtotal;
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

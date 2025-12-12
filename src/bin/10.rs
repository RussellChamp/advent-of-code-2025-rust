advent_of_code::solution!(10);

use itertools::Itertools;
use bitvec::{bitvec, vec::BitVec};

fn parse_line(line: &str) -> Option<(BitVec, Vec<BitVec>, Vec<usize>)>{
    let fields = line.split(' ').collect_vec();
    let goal: BitVec = fields[0].chars().filter_map(|c| {
        match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None
        }
    }).collect();
    // println!("{goal:?}");

    let buttons = fields[1..fields.len()-1].iter().map(|b| {
        let mut button_flag = bitvec![0; 10]; // BitVec::with_capacity(10);
        let buttons = &b[1..b.len()-1];

        // println!("{buttons:?}");
        for v in buttons.split(',').map(|v| v.parse::<usize>().unwrap()) {
            // println!("{v}");
            button_flag.insert(v, true);
        }
        button_flag
    }).collect_vec();
    let joltage_str = fields[fields.len()-1];
    let joltage = joltage_str[1..joltage_str.len()-1].split(',').map(|j| j.parse::<usize>().unwrap()).collect_vec();

    Some((goal, buttons, joltage))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_key_presses = 0;
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (goal, buttons, _) = parse_line(line).unwrap();
        // println!("goal: {goal:?}");
        // println!("buttons ({}): {buttons:?}", buttons.len());
        // println!("result: {result}");

        // cycle through all permutations of button presses
        let mut least_presses = u32::MAX;
        for ii in 0..2_usize.pow(buttons.len() as u32) {
            let mut state = goal.clone();

            for (bidx, b) in buttons.iter().enumerate() {
                if ii >> bidx & 1 == 1 {
                    state ^= b;
                }
            }

            // println!("new state {state}");
            if state.count_ones() == 0 {
                let key_presses = ii.count_ones();
                // println!("Found solution in {key_presses} key presses!");
                if key_presses < least_presses {
                    least_presses = key_presses
                }
            }
        }
        total_key_presses += least_presses;
    }

    Some(total_key_presses as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

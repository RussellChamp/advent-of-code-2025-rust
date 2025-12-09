advent_of_code::solution!(7);
use itertools::Itertools;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum State {
    Off = 0,
    On = 1
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
    let width = lines[0].len();
    let mut beams = vec![State::Off; width];

    let mut splits = 0;
    for line in lines {
        let values = line.chars().collect_vec();
        for (idx, value) in values.iter().enumerate() {
            if *value == 'S' {
                beams[idx] = State::On;
            } else if *value == '^' && beams[idx] == State::On {
                beams[idx] = State::Off;
                if idx > 0 {
                    beams[idx-1] = State::On;
                }
                if idx + 1 < width {
                    beams[idx+1] = State::On;
                }
                splits += 1;
            }
         }
        //  println!("{beams:?}");
    }
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
        let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
    let width = lines[0].len();
    let mut beams = vec![0; width];

    for line in lines {
        let values = line.chars().collect_vec();
        for (idx, value) in values.iter().enumerate() {
            if *value == 'S' {
                beams[idx] = 1;
            } else if *value == '^' && beams[idx] > 0 {
                if idx > 0 {
                    beams[idx-1] += beams[idx];
                }
                if idx + 1 < width {
                    beams[idx+1] += beams[idx];
                }
                beams[idx] = 0;
            }
         }
        //   println!("{beams:?}");
    }
    let timelines = beams.iter().sum();
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

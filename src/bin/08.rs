advent_of_code::solution!(8);
use std::fmt;
use itertools::{Itertools};

#[derive(PartialEq)]
#[derive(Clone)]
struct Position {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

fn distance_squared(pos_a: &Position, pos_b: &Position) -> u64 {
    pos_a.x.abs_diff(pos_b.x).pow(2) + pos_a.y.abs_diff(pos_b.y).pow(2) + pos_a.z.abs_diff(pos_b.z).pow(2)
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_solve(input, 1000)
}

fn part_one_solve(input: &str, max_connections: u32 ) -> Option<u64> {
    let junctions = input.lines().filter(|l| !l.is_empty()).map(|line| {
        let (x, y, z) = line.split(',').map(|v| v.parse::<u64>().unwrap()).collect_tuple().unwrap();
        Position { x, y, z }
    }).collect_vec();
    let size = junctions.len();
    // println!("Junctions: {junctions:?}");
    let mut distances = vec![];

    for a_idx in 0..size {
        for b_idx in a_idx+1..size {
            distances.push((a_idx, b_idx, distance_squared(&junctions[a_idx], &junctions[b_idx])));
        }
    }
    distances.sort_by(|(_, _, a_distance), (_, _, b_distance)| { a_distance.cmp(b_distance) });
    // println!("Distances: {distances:?}");

    let mut groups = (0..size).map(|idx| vec![idx; 1]).collect_vec();
    // println!("Groups: {groups:?}");
    // println!();


    let mut connections_made = 0;
    for (start, end, _distance) in distances {
        // println!("Considering {} => {}, {} => {} - distance {}", start, end, junctions[start], junctions[end], distance);
        // if start and end are not already connected
        let (group_a_idx, group_a) = groups.iter().find_position(|g| g.contains(&start)).unwrap();
        connections_made += 1;
        if group_a.contains(&end) {
            // println!("  Already exists in the same group! skipping");
            continue
        }

        let (group_b_idx, group_b) = groups.iter().find_position(|g| g.contains(&end)).unwrap();
        groups[group_a_idx] = [group_a.as_slice(), group_b.as_slice()].concat();
        // group_a.append(&mut group_b);
        // groups[group_a_idx] = group_a.append(group_b);
        groups.remove(group_b_idx);
        // connections_made += 1;

        // println!("Groups after {connections_made} connections: {groups:?}");
        // println!();

        //  connect them by merging their groups
        //  increment connections_made by 1
        if connections_made >= max_connections { break }
    }

    // println!("{groups:?}");

    let top3: usize = groups.iter().map(|g| g.len()).sorted().rev().take(3).product();

    // println!("top3 product {top3}");

    Some(top3 as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junctions = input.lines().filter(|l| !l.is_empty()).map(|line| {
        let (x, y, z) = line.split(',').map(|v| v.parse::<u64>().unwrap()).collect_tuple().unwrap();
        Position { x, y, z }
    }).collect_vec();
    let size = junctions.len();
    let mut distances = vec![];

    for a_idx in 0..size {
        for b_idx in a_idx+1..size {
            distances.push((a_idx, b_idx, distance_squared(&junctions[a_idx], &junctions[b_idx])));
        }
    }
    distances.sort_by(|(_, _, a_distance), (_, _, b_distance)| { a_distance.cmp(b_distance) });

    let mut groups = (0..size).map(|idx| vec![idx; 1]).collect_vec();


    let mut last_link = (0, 0);
    for (start, end, _distance) in distances {
        let (group_a_idx, group_a) = groups.iter().find_position(|g| g.contains(&start)).unwrap();
        if group_a.contains(&end) {
            continue
        }

        let (group_b_idx, group_b) = groups.iter().find_position(|g| g.contains(&end)).unwrap();
        groups[group_a_idx] = [group_a.as_slice(), group_b.as_slice()].concat();
        groups.remove(group_b_idx);

        if groups.len() == 1 {
            last_link = (start, end);
            break;
        }
    }
    let (last_junction_from, last_junction_to) = (&junctions[last_link.0], &junctions[last_link.1]);
    // println!("Last junction was between {last_junction_from:?} and {last_junction_to:?}");

    Some(last_junction_from.x * last_junction_to.x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

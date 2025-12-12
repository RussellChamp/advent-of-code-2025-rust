advent_of_code::solution!(11);
use std::collections::HashMap;
use itertools::Itertools;

fn traverse_path(nodes: &HashMap<&str, Vec<&str>>, start: &str) -> Option<usize> {
    if start == "out" { return Some(1) }
    else {
        let outputs = nodes.get(start).unwrap();
        return outputs.iter().map(|o: &&str| traverse_path(nodes, o)).sum();
    }
}

fn traverse_path_part_two(nodes: &HashMap<&str, Vec<&str>>, start: &str, end: &str, exclude_node: Option<&str>, cache: &mut HashMap<String, usize>) -> usize {
    // if we made it to our desired end node, increase the count
    if start == end {
        return 1
    }
    // if we either make it to the end node (and "end" wasn't our desired node) or we hit our excluded node, return a 0
    else if start == "out" || (!Option::is_none(&exclude_node) && start == exclude_node.unwrap()) { return 0 }
    else {
        // print!("traversing {start} => ");
        let outputs = nodes.get(start).unwrap();
        // println!("{outputs:?}");
        let mut sum = 0;
        for o in outputs.iter() {
            let key = o.to_string();
            if cache.contains_key(&key) {
                sum += cache.get(&key).unwrap();
            } else {
                let result = traverse_path_part_two(nodes, o, end, exclude_node, cache);
                cache.insert(key, result);
                sum += result;
            }
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = HashMap::new();
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
    for line in lines {
        let parts = line.split(':').collect_vec();
        let source = parts[0];
        let outputs = parts[1].trim().split(' ').collect_vec();
        nodes.insert(source, outputs);
    }
    // println!("{nodes:?}");
    let paths = traverse_path(&nodes, "you").unwrap();
    // println!("calculated paths {paths}");

    Some(paths as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut nodes = HashMap::new();
    let lines = input.lines().filter(|l| !l.is_empty()).collect_vec();
        for line in lines {
        let parts = line.split(':').collect_vec();
        let source = parts[0];
        let outputs = parts[1].trim().split(' ').collect_vec();
        nodes.insert(source, outputs);
    }

    let mut cache = HashMap::new();
    cache.clear();
    let paths_from_svr_to_dac = traverse_path_part_two(&nodes, "svr", "dac", Some("fft"), &mut cache);
    // println!("paths from svr to dac {paths_from_svr_to_dac}");

    cache.clear();
    let paths_from_dac_to_fft = traverse_path_part_two(&nodes, "dac", "fft", None, &mut cache);
    // println!("paths from dac to fft {paths_from_dac_to_fft}");

    cache.clear();
    let paths_from_fft_to_out = traverse_path_part_two(&nodes, "fft", "out", None, &mut cache);
    // println!("paths from dac to fft {paths_from_fft_to_out}");

    cache.clear();
    let paths_from_svr_to_fft = traverse_path_part_two(&nodes, "svr", "fft", Some("dac"), &mut cache);
    // println!("paths from svr to fft {paths_from_svr_to_fft}");

    cache.clear();
    let paths_from_fft_to_dac = traverse_path_part_two(&nodes, "fft", "dac", None, &mut cache);
    // println!("paths from fft to dac {paths_from_fft_to_dac}");

    cache.clear();
    let paths_from_dac_to_out = traverse_path_part_two(&nodes, "dac", "out", None, &mut cache);
    // println!("paths from fft to dac {paths_from_dac_to_out}");

    // Calculate ALL paths as follows:
    //   Paths from "svr" (START) => "dac" * "dac" => "fft" * "fft" => "out" (END)
    // + Paths from "svr" (START) => "fft" * "fft" => "dac" * "dac" => "out" (END)
    let total_paths_svr_dac_fft_out = paths_from_svr_to_dac * paths_from_dac_to_fft * paths_from_fft_to_out;
    let total_paths_svr_fft_dac_out = paths_from_svr_to_fft * paths_from_fft_to_dac * paths_from_dac_to_out;
    let total = total_paths_svr_dac_fft_out + total_paths_svr_fft_dac_out;

    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2));
    }
}

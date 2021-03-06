use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> (HashMap<String, Vec<String>>, HashMap<String, u64>) {
    let mut rel_map = HashMap::<String, Vec<String>>::new();
    let mut weight_map = HashMap::<String, u64>::new();

    let no_child_regex = Regex::new(r"([a-z]+) \((\d+)\)").unwrap();
    let with_child_regex = Regex::new(r"([a-z]+) \((\d+)\) -> (.*)").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if with_child_regex.is_match(line) {
            for capture in with_child_regex.captures_iter(line) {
                let name = capture[1].to_string();
                let weight = capture[2].parse::<u64>().unwrap();
                let children = capture[3]
                    .split(", ")
                    .map(|x| String::from(x))
                    .collect::<Vec<String>>();
                rel_map.insert(name.to_string(), children);
                weight_map.insert(name.to_string(), weight);
                break; // Regex should only match once to the line
            }
        } else {
            for capture in no_child_regex.captures_iter(line) {
                let name = capture[1].to_string();
                let weight = capture[2].parse::<u64>().unwrap();
                let children = Vec::<String>::new();
                rel_map.insert(name.to_string(), children);
                weight_map.insert(name.to_string(), weight);
                break; // Regex should only match once to the line
            }
        }
    }
    return (rel_map, weight_map);
}

#[aoc(day7, part1)]
fn solve_part_1(input: &(HashMap<String, Vec<String>>, HashMap<String, u64>)) -> String {
    let mut all_children = HashSet::<String>::new();
    let mut all_names = HashSet::<String>::new();
    // Extract all node names and all nodes that are child to another node
    for (parent, children) in &input.0 {
        all_names.insert(parent.to_string());
        for child in children {
            all_children.insert(child.to_string());
        }
    }
    // Determine the bottom node (not a child to any other node)
    for name in all_names {
        if !all_children.contains(&name) {
            return name;
        }
    }
    panic!("D07_P1 - should not get here!");
}

#[aoc(day7, part2)]
fn solve_part_2(input: &(HashMap<String, Vec<String>>, HashMap<String, u64>)) -> u64 {
    let bottom_node = solve_part_1(input);
    let mut weight_map = input.1.clone();
    let mut corrected_weights: Vec<u64> = vec![];
    let _total_weight = calculate_program_weight(
        &bottom_node,
        &input.0,
        &mut weight_map,
        &mut corrected_weights,
    );
    return corrected_weights[0];
}

fn calculate_program_weight(
    name: &String,
    rel_map: &HashMap<String, Vec<String>>,
    weight_map: &mut HashMap<String, u64>,
    corrected_weights: &mut Vec<u64>,
) -> u64 {
    if rel_map.get(name).unwrap().is_empty() {
        return *weight_map.get(name).unwrap();
    }
    let mut total_weight = 0;
    let mut weights_seen = HashMap::<u64, Vec<String>>::new();
    total_weight += weight_map.get(name).unwrap();
    for child in rel_map.get(name).unwrap() {
        let child_weight = calculate_program_weight(child, rel_map, weight_map, corrected_weights);
        total_weight += child_weight;
        if weights_seen.contains_key(&child_weight) {
            weights_seen
                .get_mut(&child_weight)
                .unwrap()
                .push(child.to_string());
        } else {
            weights_seen.insert(child_weight, vec![child.to_string()]);
        }
    }
    // weight_map.insert(name.to_string(), total_weight);
    if weights_seen.keys().len() > 1 {
        // We have a mismatch
        // Determine which node is the mismatch
        for (key, value) in &weights_seen {
            if value.len() == 1 {
                // Calculate the good total weight out of all child nodes
                let mut weights = weights_seen.keys().collect::<Vec<&u64>>();
                weights.retain(|x| *x != key);
                let good_weight = weights[0];
                // Calculate the weight the relevant child node should be so all towers are balanced
                let delta = *good_weight as i64 - *key as i64;
                let correct_weight = *weight_map.get(&value[0]).unwrap() as i64 + delta;
                corrected_weights.push(correct_weight as u64);
                break;
            }
        }
    }
    return total_weight;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d07_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day7.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("hlqnsbe", result);
    }

    #[test]
    fn test_d07_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day7.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1993, result);
    }
}

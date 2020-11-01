use regex::Regex;

use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day7)]
fn generate_input(input: &str) -> (HashMap::<String, Vec<String>>, HashMap<String, u64>) {
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
                let children = capture[3].split(", ").map(|x| String::from(x)).collect::<Vec<String>>();
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
fn solve_part_1(input: &(HashMap::<String, Vec<String>>, HashMap<String, u64>)) -> String {
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
fn solve_part_2(input: &(HashMap::<String, Vec<String>>, HashMap<String, u64>)) -> String {
    unimplemented!();
}

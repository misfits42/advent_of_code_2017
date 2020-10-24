use regex::Regex;

use std::collections::HashSet;

struct ProgramNode {
    name: String,
    weight: u64,
    children: Vec<String>
}

impl ProgramNode {
    pub fn new(name: String, weight: u64, children: Vec<String>) -> Self {
        Self {
            name: name,
            weight: weight,
            children: children.to_vec()
        }
    }

    pub fn get_name(&self) -> String {
        return self.name.to_owned();
    }

    pub fn get_weight(&self) -> u64 {
        return self.weight;
    }

    pub fn get_children(&self) -> Vec<String> {
        return self.children.to_vec();
    }

    pub fn get_num_children(&self) -> usize {
        return self.children.len();
    }
}

#[aoc_generator(day7)]
fn generate_input(input: &str) -> Vec<ProgramNode> {
    let mut nodes = Vec::<ProgramNode>::new();
    let no_child_regex = Regex::new(r"([a-z]+) \((\d+)\)").unwrap();
    let with_child_regex = Regex::new(r"([a-z]+) \((\d+)\) -> (.*)").unwrap();
    for line in input.lines() {
        let line = line.trim();
        if with_child_regex.is_match(line) {
            for capture in with_child_regex.captures_iter(line) {
                let name = capture[1].to_owned();
                let weight = capture[2].parse::<u64>().unwrap();
                let children = capture[3].split(", ").map(|x| String::from(x)).collect::<Vec<String>>();
                // println!("{:?}", children);
                nodes.push(ProgramNode::new(name, weight, children));
                break; // Regex should only match once to the line
            }
        } else {
            for capture in no_child_regex.captures_iter(line) {
                let name = capture[1].to_owned();
                let weight = capture[2].parse::<u64>().unwrap();
                let children = Vec::<String>::new();
                nodes.push(ProgramNode::new(name, weight, children));
                break; // Regex should only match once to the line
            }
        }
    }
    return nodes;
}

#[aoc(day7, part1)]
fn solve_part_1(input: &Vec<ProgramNode>) -> String {
    let mut all_children = HashSet::<String>::new();
    let mut all_names = Vec::<String>::new();
    // Find out all names and all nodes that are a child to another node
    for node in input {
        all_names.push(node.get_name());
        for child in node.get_children() {
            all_children.insert(child);
        }
    }
    // Look for the bottom node - the one that is not a child to another node
    for name in all_names {
        if !all_children.contains(&name) {
            return name;
        }
    }
    panic!("D07_P1 - should not get here!");
}

#[aoc(day7, part2)]
fn solve_part_2(input: &Vec<ProgramNode>) -> String {
    unimplemented!();
}

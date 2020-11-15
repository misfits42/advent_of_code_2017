use std::collections::HashMap;
use std::collections::HashSet;

use regex::Regex;

#[aoc_generator(day12)]
fn generate_input(input: &str) -> HashMap<String, HashSet<String>> {
    // Create regex to match and extract data fields from lines in input
    let line_regex = Regex::new(r"(\d+) <-> (.*)").unwrap();
    // Create HashMap to record relationship between programs in village
    let mut program_conns = HashMap::<String, HashSet<String>>::new();
    let input = input.trim();
    for line in input.lines() {
        let line = line.trim();
        for capture in line_regex.captures_iter(line) {
            // let program = capture[1].to_string();
            let connected = capture[2].split(", ");
            // If not already in output, initialise entry for the current program
            if !program_conns.contains_key(&capture[1]) {
                program_conns.insert(capture[1].to_string(), HashSet::new());
            }
            // For each connected program, add connection relationship
            for conn in connected {
                if !program_conns.contains_key(conn) {
                    program_conns.insert(conn.to_string(), HashSet::new());
                }
                program_conns
                    .get_mut(&capture[1])
                    .unwrap()
                    .insert(conn.to_string());
                program_conns
                    .get_mut(conn)
                    .unwrap()
                    .insert(capture[1].to_string());
            }
        }
    }
    return program_conns;
}

#[aoc(day12, part1)]
fn solve_part_1(input: &HashMap<String, HashSet<String>>) -> usize {
    // Initialise set used to record all programs in same group as "0"
    let start_program = String::from("0");
    let mut zero_group: HashSet<String> = HashSet::new();
    zero_group.insert(String::from("0"));
    // Recursively process individual program conns to determine which are in same group as "0"
    find_group_conns(input, &start_program, &mut zero_group);
    // Return number of programs in same group as "0" program
    return zero_group.len();
}

#[aoc(day12, part2)]
fn solve_part_2(input: &HashMap<String, HashSet<String>>) -> usize {
    // Initialise count for total number of groups
    let mut total_groups = 0;
    // Make mutable collection used to mark off programs that have been grouped together
    let mut remaining_progs = input.keys().map(|x| x.to_string()).collect::<HashSet<String>>();
    loop {
        // Check if we have exhausted all groups
        if remaining_progs.len() == 0 {
            return total_groups;
        }
        // We have at least one more group, so increment total count
        total_groups += 1;
        // Initialise current group
        let mut group: HashSet<String> = HashSet::new();
        let start_prog = remaining_progs.iter().next().unwrap();
        group.insert(start_prog.to_string());
        // Recursively process connections to determine contents of current group
        find_group_conns(input, &start_prog, &mut group);
        // Remove connected programs from remaining programs
        for prog in group {
            remaining_progs.remove(&prog);
        }
    }
}

/// Processes the program connections to determine connections within same group.
fn find_group_conns(
    conns: &HashMap<String, HashSet<String>>,
    prog: &String,
    group: &mut HashSet<String>,
) {
    // Get connections for current program
    for connected in conns.get(prog).unwrap() {
        // If connection not already found, record connection and add its connections to group
        if !group.contains(connected) {
            group.insert(connected.clone());
            find_group_conns(conns, connected, group);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d12_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day12.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(288, result);
    }

    #[test]
    fn test_d12_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day12.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(211, result);
    }
}

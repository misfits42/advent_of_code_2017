use std::collections::HashMap;
use std::collections::VecDeque;

use regex::Regex;

enum DanceMove {
    Spin { shift: u64 },
    Exchange { pos_1: usize, pos_2: usize },
    Partner { prog_1: String, prog_2: String },
}

#[aoc_generator(day16)]
fn generate_input(input: &str) -> Vec<DanceMove> {
    let input = input.trim();
    // Create regexes to match and extract fields from raw dance move inputs
    let spin_regex = Regex::new(r"s(\d+)").unwrap();
    let exchange_regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
    let partner_regex = Regex::new(r"p([a-p])/([a-p])").unwrap();
    let mut dance_moves = Vec::<DanceMove>::new();
    for move_raw in input.split(",") {
        if move_raw.starts_with("s") {
            // Found a spin move
            for capture in spin_regex.captures_iter(move_raw) {
                let shift = capture[1].parse::<u64>().unwrap();
                let spin_move = DanceMove::Spin { shift: shift };
                dance_moves.push(spin_move);
            }
        } else if move_raw.starts_with("x") {
            // Found an exchange move
            for capture in exchange_regex.captures_iter(move_raw) {
                let pos_1 = capture[1].parse::<usize>().unwrap();
                let pos_2 = capture[2].parse::<usize>().unwrap();
                let exchange_move = DanceMove::Exchange {
                    pos_1: pos_1,
                    pos_2: pos_2,
                };
                dance_moves.push(exchange_move);
            }
        } else if move_raw.starts_with("p") {
            // Found a partner move
            for capture in partner_regex.captures_iter(move_raw) {
                let prog_1 = capture[1].to_string();
                let prog_2 = capture[2].to_string();
                let partner_move = DanceMove::Partner {
                    prog_1: prog_1,
                    prog_2: prog_2,
                };
                dance_moves.push(partner_move);
            }
        }
    }
    return dance_moves;
}

#[aoc(day16, part1)]
fn solve_part_1(input: &Vec<DanceMove>) -> String {
    // Initialise the programs in the starting order
    let mut programs = get_initial_programs();
    // Process each dance move
    conduct_dance_round(input, &mut programs);
    // Generate representation of final order of programs after all dance moves executed
    return generate_programs_to_string(&programs);
}

/// Solves AoC 2017 Day 16, Part 2.
#[aoc(day16, part2)]
fn solve_part_2(input: &Vec<DanceMove>) -> String {
    // Generate initial ordering of programs
    let mut programs = get_initial_programs();
    let mut output_store = HashMap::<u64, String>::new();
    // Program ordering after dance round repeats with cycle of duration 60 rounds
    for round in 1..=60 {
        conduct_dance_round(input, &mut programs);
        let program_order = generate_programs_to_string(&programs);
        output_store.insert(round, program_order);
    }
    // Determine which of the program orders will be present after the one billionth dance round
    let index: u64 = 1000000000 % 60;
    return output_store.get(&index).unwrap().to_string();
}

/// Generates representation of order of programs.
fn generate_programs_to_string(programs: &VecDeque<String>) -> String {
    let mut output = String::from("");
    for program in programs {
        output += &program;
    }
    return output;
}

/// Conducts a single round of dancing, using the specified dance moves to change the order of the
/// programs.
fn conduct_dance_round(dance_moves: &Vec<DanceMove>, programs: &mut VecDeque<String>) {
    // Process each dance move
    for dance_move in dance_moves {
        match dance_move {
            // Remove program from end and add to front for "shift" number of times
            DanceMove::Spin{shift} => {
                for _ in 0..*shift {
                    let popped = programs.pop_back().unwrap();
                    programs.push_front(popped);
                }
            },
            // Swap the programs in the two positions specified
            DanceMove::Exchange{pos_1, pos_2} => {
                programs.swap(*pos_1, *pos_2);
            },
            // Swap the location of the two specified programs
            DanceMove::Partner{prog_1, prog_2} => {
                let pos_1 = programs.iter().position(|x| x == prog_1).unwrap();
                let pos_2 = programs.iter().position(|x| x == prog_2).unwrap();
                programs.swap(pos_1, pos_2);
            }
        }
    }
}

/// Generates the initial order of programs before executing any dance moves.
fn get_initial_programs() -> VecDeque<String> {
    // Initialise the programs in the starting order
    let programs = VecDeque::from(
        "abcdefghijklmnop"
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    );
    return programs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d16_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day16.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("pkgnhomelfdibjac", result);
    }

    #[test]
    fn test_d16_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day16.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!("pogbjfihclkemadn", result);
    }
}

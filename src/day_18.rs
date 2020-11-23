use super::utils::machines::{SoundComputer, SoundComputerInstruction};

use regex::Regex;

#[aoc_generator(day18)]
fn generate_input(input: &str) -> Vec<SoundComputerInstruction> {
    let input = input.trim();
    let mut instructions: Vec<SoundComputerInstruction> = vec![];
    // Create regexes to match various instruction formats
    let snd_regex = Regex::new(r"snd (.*)").unwrap();
    let set_regex = Regex::new(r"set (.*) (.*)").unwrap();
    let add_regex = Regex::new(r"add (.*) (.*)").unwrap();
    let mul_regex = Regex::new(r"mul (.*) (.*)").unwrap();
    let mod_regex = Regex::new(r"mod (.*) (.*)").unwrap();
    let rcv_regex = Regex::new(r"rcv (.*)").unwrap();
    let jgz_regex = Regex::new(r"jgz (.*) (.*)").unwrap();
    // Process each line of input and extract instruction details
    for line in input.lines() {
        let line = line.trim();
        if snd_regex.is_match(line) {
            let captures = snd_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            instructions.push(SoundComputerInstruction::Snd{value_1: value_1});
        } else if set_regex.is_match(line) {
            let captures = set_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            let value_2 = captures[2].to_string();
            instructions.push(SoundComputerInstruction::Set{value_1: value_1, value_2: value_2});
        } else if add_regex.is_match(line) {
            let captures = add_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            let value_2 = captures[2].to_string();
            instructions.push(SoundComputerInstruction::Add{value_1: value_1, value_2: value_2});
        } else if mul_regex.is_match(line) {
            let captures = mul_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            let value_2 = captures[2].to_string();
            instructions.push(SoundComputerInstruction::Mul{value_1: value_1, value_2: value_2});
        } else if mod_regex.is_match(line) {
            let captures = mod_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            let value_2 = captures[2].to_string();
            instructions.push(SoundComputerInstruction::Mod{value_1: value_1, value_2: value_2});
        } else if rcv_regex.is_match(line) {
            let captures = rcv_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            instructions.push(SoundComputerInstruction::Rcv{value_1: value_1});
        } else if jgz_regex.is_match(line) {
            let captures = jgz_regex.captures(line).unwrap();
            let value_1 = captures[1].to_string();
            let value_2 = captures[2].to_string();
            instructions.push(SoundComputerInstruction::Jgz{value_1: value_1, value_2: value_2});
        } else {
            panic!("Day 18 generator - should not get here!");
        }
    }
    return instructions;
}

#[aoc(day18, part1)]
fn solve_part_1(instructions: &Vec<SoundComputerInstruction>) -> i64 {
    let mut sound_comp = SoundComputer::new(instructions);
    sound_comp.execute_single_mode();
    return *sound_comp.last_recovered_freq().unwrap();
}

#[aoc(day18, part2)]
fn solve_part_2(instructions: &Vec<SoundComputerInstruction>) -> usize {
    // Create two separate sound computers with program IDs initialise
    let mut sound_comp_0 = SoundComputer::new(instructions);
    sound_comp_0.update_register('p', 0);
    let mut sound_comp_1 = SoundComputer::new(instructions);
    sound_comp_1.update_register('p', 1);
    loop {
        // Check if stopping condition has been met
        if sound_comp_0.is_halted() && sound_comp_1.is_halted() {
            return sound_comp_1.snd_count();
        } else if sound_comp_0.is_halted() && sound_comp_1.is_awaiting_input() {
            return sound_comp_1.snd_count();
        } else if sound_comp_0.is_awaiting_input() && sound_comp_1.is_halted() {
            return sound_comp_1.snd_count();
        } else if sound_comp_0.is_awaiting_input() && sound_comp_1.is_awaiting_input() {
            return sound_comp_1.snd_count();
        }
        // Execute next instruction for both SoundComputers and store output
        let output_from_0 = sound_comp_0.execute_double_mode();
        let output_from_1 = sound_comp_1.execute_double_mode();
        // Push input values to both SoundComputers if available
        if output_from_1.is_some() {
            sound_comp_0.push_input(output_from_1.unwrap());
        }
        if output_from_0.is_some() {
            sound_comp_1.push_input(output_from_0.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d18_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day18.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(3188, result);
    }

    #[test]
    fn test_d18_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day18.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(7112, result);
    }
}

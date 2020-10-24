use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
fn generate_input(input: &str) -> Vec<u64> {
    let mut memory_banks = Vec::<u64>::new();
    for num in input.trim().split_whitespace() {
        memory_banks.push(num.parse::<u64>().unwrap());
    }
    return memory_banks;
}

#[aoc(day6, part1)]
fn solve_part_1(input: &Vec<u64>) -> u64 {
    let mut memory_banks = input.to_vec();
    let mut cycles_done = 0;
    let mut states_seen = HashSet::<Vec<u64>>::new();
    // Record state hash of initial state
    states_seen.insert(memory_banks.to_vec());
    loop {
        cycles_done += 1;
        // Find memory bank with most blocks
        let mut target_index = 0;
        let mut max_value = 0;
        for i in 0..memory_banks.len() {
            if memory_banks[i] > max_value {
                max_value = memory_banks[i];
                target_index = i;
            }
        }
        // Take out blocks from largest bank and reallocate
        let mut blocks_to_alloc = memory_banks[target_index];
        memory_banks[target_index] = 0;
        let mut index = target_index;
        while blocks_to_alloc > 0 {
            // Move index forward and wrap around to start if needed
            index += 1;
            index = index % memory_banks.len();
            // Reallocate a memory block
            memory_banks[index] += 1;
            blocks_to_alloc -= 1;
        }
        // Check if the state has been seen before
        let is_new_state = states_seen.insert(memory_banks.to_vec());
        if !is_new_state {
            return cycles_done;
        }
    }
}

#[aoc(day6, part2)]
fn solve_part_2(input: &Vec<u64>) -> u64 {
    let mut memory_banks = input.to_vec();
    let mut cycles_done = 0;
    let mut states_seen = HashMap::<Vec<u64>, u64>::new();
    // Record state hash of initial state
    states_seen.insert(memory_banks.to_vec(), cycles_done);
    loop {
        cycles_done += 1;
        // Find memory bank with most blocks
        let mut target_index = 0;
        let mut max_value = 0;
        for i in 0..memory_banks.len() {
            if memory_banks[i] > max_value {
                max_value = memory_banks[i];
                target_index = i;
            }
        }
        // Take out blocks from largest bank and reallocate
        let mut blocks_to_alloc = memory_banks[target_index];
        memory_banks[target_index] = 0;
        let mut index = target_index;
        while blocks_to_alloc > 0 {
            // Move index forward and wrap around to start if needed
            index += 1;
            index = index % memory_banks.len();
            // Reallocate a memory block
            memory_banks[index] += 1;
            blocks_to_alloc -= 1;
        }
        // Check if the state has been seen before
        if states_seen.contains_key(&memory_banks) {
            return cycles_done - states_seen.get(&memory_banks).unwrap();
        } else {
            states_seen.insert(memory_banks.to_vec(), cycles_done);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d06_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day6.txt").unwrap().trim());
        let result = solve_part_1(&input);
        assert_eq!(7864, result);
    }

    #[test]
    fn test_d06_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day6.txt").unwrap().trim());
        let result = solve_part_2(&input);
        assert_eq!(1695, result);
    }
}

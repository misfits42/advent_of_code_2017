use std::collections::VecDeque;

use regex::Regex;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Generator {
    current_value: u64,
    factor: u64,
    divisor: u64
}

impl Generator {
    pub fn new(initial_value: u64, factor: u64, divisor: u64) -> Self {
        Self {
            current_value: initial_value,
            factor: factor,
            divisor: divisor
        }
    }

    /// Gets the most recent value calculated by the generator.
    pub fn get_current_value(&self) -> u64 {
        return self.current_value;
    }

    /// Calculates the next value and sets the current value to it.
    pub fn generate_next_value(&mut self) {
        let next_value = (self.current_value * self.factor) % self.divisor;
        self.current_value = next_value;
    }

    // Returns the last two bytes of the generator current value as a u16.
    pub fn get_check_value(&self) -> u16 {
        return (self.current_value & 0xffff) as u16;
    }
}

#[aoc_generator(day15)]
fn generate_input(input: &str) -> (Generator, Generator) {
    let input = input.trim();
    let mut generators = Vec::<Generator>::new();
    // Create regexes to match and extract generator initial values
    let gen_a_regex = Regex::new(r"Generator A starts with (\d+)").unwrap();
    let gen_b_regex = Regex::new(r"Generator B starts with (\d+)").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    // Generator A
    for capture_a in gen_a_regex.captures_iter(lines[0]) {
        let gen_a_initial = capture_a[1].parse::<u64>().unwrap();
        let generator_a = Generator::new(gen_a_initial, 16807, 2147483647);
        generators.push(generator_a);
    }
    // Generator B
    for capture_b in gen_b_regex.captures_iter(lines[1]) {
        let gen_b_initial = capture_b[1].parse::<u64>().unwrap();
        let generator_b = Generator::new(gen_b_initial, 48271, 2147483647);
        generators.push(generator_b);
    }
    return (generators[0], generators[1]);
}

#[aoc(day15, part1)]
fn solve_part_1(input: &(Generator, Generator)) -> u64 {
    let mut judge_score = 0;
    let mut generator_a = input.0;
    let mut generator_b = input.1;
    for _ in 0..40000000 { // Conduct 40 million rounds
        // Generate next value for both generators
        generator_a.generate_next_value();
        generator_b.generate_next_value();
        // Compare check bytes from both generators - if matching, increment judge score
        if generator_a.get_check_value() == generator_b.get_check_value() {
            judge_score += 1;
        }
    }
    return judge_score;
}

#[aoc(day15, part2)]
fn solve_part_2(input: &(Generator, Generator)) -> u64 {
    let mut judge_score = 0;
    let mut pairs_observed = 0;
    let mut generator_a = input.0;
    let mut generator_b = input.1;
    // Record check bytes from acceptable values from each generator in a queue
    let mut gen_a_check_values = VecDeque::<u16>::new();
    let mut gen_b_check_values = VecDeque::<u16>::new();
    loop {
        // Terminate only when 5 million pairs have been compared
        if pairs_observed >= 5000000 {
            return judge_score;
        }
        // Generate next value for both generators
        generator_a.generate_next_value();
        generator_b.generate_next_value();
        // Check if current generator values are acceptable
        if generator_a.get_current_value() % 4 == 0 {
            gen_a_check_values.push_back(generator_a.get_check_value());
        }
        if generator_b.get_current_value() % 8 == 0 {
            gen_b_check_values.push_back(generator_b.get_check_value());
        }
        // Check if there is a pair of values available to compare
        if !gen_a_check_values.is_empty() && !gen_b_check_values.is_empty() {
            // Retrieve next value from each generator to compare
            let check_a = gen_a_check_values.pop_front();
            let check_b = gen_b_check_values.pop_front();
            if check_a == check_b {
                judge_score += 1;
            }
            pairs_observed += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d15_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day15.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(594, result);
    }

    #[test]
    fn test_d15_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day15.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(328, result);
    }
}

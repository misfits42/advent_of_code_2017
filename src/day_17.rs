use super::utils::collections::Spinlock;

#[aoc_generator(day17)]
fn generate_input(input: &str) -> usize {
    return input.trim().parse::<usize>().unwrap();
}

#[aoc(day17, part1)]
fn solve_part_1(input: &usize) -> usize {
    // Create a new spinlock
    let skip_size = *input;
    let mut spinlock = Spinlock::new(skip_size);
    // Conduct insertions into spinlock
    for value in 1..=2017 {
        spinlock.skip_forward();
        spinlock.insert_after_cursor(value);
    }
    // Return the value after the last value inserted into the spinlock
    return spinlock.peek_after_cursor();
}

#[aoc(day17, part2)]
fn solve_part_2(input: &usize) -> usize {
    // Initialise values to keep track of result
    let mut value_after_0: usize = 0;
    let skip_size = *input;
    let mut cursor = 0;
    for value in 1..=50000000 {
        // Skip cursor forward within circular buffer and move to new insert index
        cursor = (cursor + skip_size) % value + 1;
        // Check if a new value would be inserted directly after the value 0 - which remains at
        // index 0 due to implementation of the spinlock
        if cursor == 1 {
            value_after_0 = value;
        }
    }
    return value_after_0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d17_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day17.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1642, result);
    }

    #[test]
    fn test_d17_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day17.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(33601318, result);
    }
}

use super::utils::collections::Spinlock;

#[aoc_generator(day17)]
fn generate_input(input: &str) -> usize {
    return input.trim().parse::<usize>().unwrap();
}

#[aoc(day17, part1)]
fn solve_part_1(input: &usize) -> i64 {
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
fn solve_part_2(input: &usize) -> i64 {
    // Create a new spinlock
    let skip_size = *input;
    let mut spinlock = Spinlock::new(skip_size);
    // Conduct insertions into spinlock
    for value in 1..=50000000 {
        if value % 10000 == 0 {
            println!("Inserting value {} ...", value);
        }
        spinlock.skip_forward();
        spinlock.insert_after_cursor(value);
    }
    // Return the value in location after 0 value, which will be in index 0
    return spinlock.peek_after_index(0).unwrap();
}

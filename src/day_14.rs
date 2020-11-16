use super::utils::hash::*;

#[aoc_generator(day14)]
fn generate_input(input: &str) -> String {
    return input.trim().to_string();
}

#[aoc(day14, part1)]
fn solve_part_1(input: &String) -> u32 {
    // Initialise count of number of squares used
    let mut squares_used = 0;
    for i in 0..=127 {
        let hash_input = format!("{}-{}", input, i.to_string());
        let knot_hash = calculate_knot_hash(&hash_input);
        for value in knot_hash {
            squares_used += value.count_ones();
        }
    }
    return squares_used;
}

#[aoc(day14, part2)]
fn solve_part_2(input: &String) -> u32 {
    unimplemented!();
}

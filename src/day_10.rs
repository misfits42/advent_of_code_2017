use super::utils::hash::*;

#[aoc_generator(day10)]
fn generate_input(input: &str) -> String {
    return String::from(input.trim());
}

#[aoc(day10, part1)]
fn solve_part_1(input: &String) -> u64 {
    // Convert the input into list of lengths
    let lengths = input.split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    // Initialise the list, current position and skip size
    let mut list = (0..=255).collect::<Vec<u8>>();
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;
    // Process each length
    list = calculate_sparse_hash(&mut list, &lengths, &mut current_pos, &mut skip_size);
    // Return the product of the first two values in the list after processing
    return list[0] as u64 * list[1] as u64;
}

#[aoc(day10, part2)]
fn solve_part_2(input: &String) -> String {
    let knot_hash = calculate_knot_hash(input);
    // Convert dense hash to hexadecimal representation
    let hex_output = hex::encode(knot_hash);
    return hex_output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d10_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day10.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(38628, result);
    }

    #[test]
    fn test_d10_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day10.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!("e1462100a34221a7f0906da15c1c979a", result);
    }
}

#[aoc_generator(day10)]
fn generate_input(input: &str) -> String {
    return String::from(input.trim());
}

fn perform_knot_hash_round(
    list: &mut Vec<u8>,
    lengths: &Vec<usize>,
    current_pos: &mut usize,
    skip_size: &mut usize,
) {
    // Process each length
    for length in lengths {
        // Calculate list of indices included in the reverse
        let mut rev_indices = Vec::<usize>::new();
        for i in 0..*length {
            let index = (*current_pos + i) % list.len();
            &rev_indices.push(index);
        }
        // Extract values from the list then reverse
        let mut extract_list = Vec::<u8>::new();
        for i in &rev_indices {
            extract_list.push(list[*i]);
        }
        extract_list.reverse();
        // Replace values in list with reversed values
        for i in 0..extract_list.len() {
            let list_index = rev_indices[i];
            list[list_index] = extract_list[i]
        }
        // Move the current positon
        *current_pos = (*current_pos + length + *skip_size) % list.len();
        // Increase skip size by 1
        *skip_size += 1;
    }
}

#[aoc(day10, part1)]
fn solve_part_1(input: &String) -> u64 {
    // Convert the input into list of lengths
    let lengths = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    // Initialise the list, current position and skip size
    let mut list = (0..=255).collect::<Vec<u8>>();
    let mut current_pos: usize = 0;
    let mut skip_size = 0;
    // Process each length
    perform_knot_hash_round(&mut list, &lengths, &mut current_pos, &mut skip_size);
    // Return the product of the first two values in the list after processing
    return list[0] as u64 * list[1] as u64;
}

#[aoc(day10, part2)]
fn solve_part_2(input: &String) -> u64 {
    unimplemented!();
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
}

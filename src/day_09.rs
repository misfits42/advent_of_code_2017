#[aoc_generator(day9)]
fn generate_input(input: &str) -> Vec<char> {
    let output = input.trim().chars().collect::<Vec<char>>();
    return output;
}

#[aoc(day9, part1)]
fn solve_part_1(input: &Vec<char>) -> u64 {
    // Step through characters in input stream, going down one level when new nested group is found
    let mut total_score = 0;
    let mut garb_count = 0;
    let mut index = 0;
    process_stream(input, &mut index, 0, &mut total_score, &mut garb_count);
    return total_score;
}

/// Processes the observed stream of characters to calculate total group score and total
/// non-cancelled garbage characters.
fn process_stream(char_stream: &Vec<char>, index: &mut usize, depth: u64, total_score: &mut u64, garb_count: &mut u64) {
    loop {
        if *index >= char_stream.len() { // Check if end of char stream has been reached
            return;
        }
        if char_stream[*index] == '{' { // Start of new group - go down one level
            *index += 1;
            process_stream(char_stream, index, depth + 1, total_score, garb_count);
        } else if char_stream[*index] == '}' { // End of a group - increase total score
            *index += 1;
            *total_score += depth;
            return;
        } else if char_stream[*index] == '<' { // Consume garbage
            *index += 1; // Start garbage checking on next character after start of garbage
            loop { // Keep processing garbage until end of current garbage is reached
                if char_stream[*index] == '>' { // Reached end of garbage
                    *index += 1;
                    break;
                } else if char_stream[*index] == '!' { // Next character is cancelled, so skip it
                    *index += 2;
                } else { // Count the non-cancelled garbage character
                    *garb_count += 1;
                    *index += 1;
                }
            }
        } else if char_stream[*index] == ',' { // Found a separator - advance to next character
            *index += 1;
        }
    }
}

#[aoc(day9, part2)]
fn solve_part_2(input: &Vec<char>) -> u64 {
    // Step through characters in input stream, going down one level when new nested group is found
    let mut total_score = 0;
    let mut garb_count = 0;
    let mut index = 0;
    process_stream(input, &mut index, 0, &mut total_score, &mut garb_count);
    return garb_count;
}

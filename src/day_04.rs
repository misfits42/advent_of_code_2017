use std::collections::HashSet;

#[aoc_generator(day4)]
fn generate_input(input: &str) -> Vec<String> {
    let mut passphrases = Vec::<String>::new();
    for line in input.lines() {
        passphrases.push(String::from(line.trim()));
    }
    return passphrases;
}

#[aoc(day4, part1)]
fn solve_part_1(input: &Vec<String>) -> u64 {
    let mut valid_count: u64 = 0;
    // Check each of the provided passphrases
    for passphrase in input {
        // Keep track of each word seen so far for each passphrase
        let mut words = HashSet::<String>::new();
        let mut valid_passphrase = true;
        for word in passphrase.split(" ") {
            // Check if the word has already been seen in the passphrase
            if words.contains(word) {
                valid_passphrase = false;
                break;
            }
            words.insert(String::from(word));
        }
        // If we have a valid passphrase, increment the count by 1
        if valid_passphrase {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[aoc(day4, part2)]
fn solve_part_2(input: &Vec<String>) -> u64 {
    unimplemented!();
}

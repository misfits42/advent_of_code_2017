use std::collections::HashMap;
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
    let mut valid_count: u64 = 0;
    // Check each of the provided passphrases
    for passphrase in input {
        // Keep track of all the words seen so far for each passphrase
        let mut words = HashSet::<String>::new();
        let mut valid_passphrase = true;
        for word in passphrase.split(" ") {
            // Check if current word or an anagram of it has already been seen
            if words.contains(word) || conduct_anagram_check(&words, &String::from(word)) {
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

fn conduct_anagram_check(words: &HashSet<String>, test_word: &String) -> bool {
    for word in words {
        // If two words are not the same length, they cannot be anagrams of each other
        if test_word.len() != word.len() {
            continue;
        }
        let mut char_map = generate_char_map(word);
        // Try each character in the test word
        for c in test_word.chars() {
            // If test character is in word, decrement character count and remove if 0 remain
            if char_map.contains_key(&c) {
                *char_map.get_mut(&c).unwrap() -= 1;
                if *char_map.get(&c).unwrap() == 0 {
                    char_map.remove(&c);
                }
            }
        }
        // If all characters were match, an anagram pair has been found
        if char_map.is_empty() {
            return true;
        }
    }
    return false;
}

// Generates a HashMap recording how many of each character appear in the given word.
fn generate_char_map(word: &String) -> HashMap<char, u64> {
    let mut char_map = HashMap::<char, u64>::new();
    for c in word.chars() {
        if !char_map.contains_key(&c) {
            char_map.insert(c, 1);
        } else {
            *char_map.get_mut(&c).unwrap() += 1;
        }
    }
    return char_map;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d04_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day4.txt").unwrap().trim());
        let result = solve_part_1(&input);
        assert_eq!(386, result);
    }

    #[test]
    fn test_d04_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day4.txt").unwrap().trim());
        let result = solve_part_2(&input);
        assert_eq!(208, result);
    }
}

#[aoc_generator(day1)]
fn generate_input(input: &str) -> Vec<u32> {
    return input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
}

#[aoc(day1, part1)]
fn solve_part_1(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for i in 0..input.len() {
        // Get the current digit and next digit.
        let next = (i + 1) % input.len();
        let digit = input[i];
        let next_digit = input[next];
        if digit == next_digit {
            sum += digit;
        }
    }
    return sum;
}

#[aoc(day1, part2)]
fn solve_part_2(input: &Vec<u32>) -> u32 {
    let mut sum = 0;

    for i in 0..input.len() {
        let next = (i + input.len() / 2) % input.len();
        let digit = input[i];
        let next_digit = input[next];
        if digit == next_digit {
            sum += digit;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d01_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day1.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(1150, result);
    }

    #[test]
    fn test_d01_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day1.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1064, result);
    }
}

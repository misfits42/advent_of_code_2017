#[aoc_generator(day2)]
fn generate_input(input: &str) -> Vec<Vec<u64>> {
    let mut rows: Vec<Vec<u64>> = vec![];
    for line in input.lines() {
        let numbers = line.trim().split_whitespace();
        rows.push(numbers.map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
    }
    return rows;
}

#[aoc(day2, part1)]
fn solve_part_1(input: &Vec<Vec<u64>>) -> u64 {
    let mut checksum = 0;
    for row in input {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        checksum += max - min;
    }
    return checksum;
}

#[aoc(day2, part2)]
fn solve_part_2(input: &Vec<Vec<u64>>) -> u64 {
    let mut sum = 0;

    for row in input {
        // For each row, check each pair of values
        for i in 0..row.len() {
            for j in (i+1)..row.len() {
                let first = row[i];
                let second = row[j];
                // Check if values divide evenly into one another - if one is a factor of the other
                if first % second == 0 {
                    sum += first / second;
                    break;
                } else if second % first == 0 {
                    sum += second / first;
                    break;
                }
            }
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d02_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day2.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(45158, result);
    }

    #[test]
    fn test_d02_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day2.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(294, result);
    }
}
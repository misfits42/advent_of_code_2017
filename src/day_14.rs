use std::collections::HashMap;

use super::utils::hash::*;
use super::utils::map::Point2D;

#[aoc_generator(day14)]
fn generate_input(input: &str) -> String {
    return input.trim().to_string();
}

#[aoc(day14, part1)]
fn solve_part_1(input: &String) -> u64 {
    // Initialise count of number of squares used
    let mut squares_used = 0;
    // Calculate knot hash for each of 128 rows in grid
    for i in 0..=127 {
        // Generate hash input for current row
        let hash_input = format!("{}-{}", input, i.to_string());
        // Calculate the knot hash, output being 16 u8 values
        let knot_hash = calculate_knot_hash(&hash_input);
        // Count how many squares are used in the current row
        for value in knot_hash {
            squares_used += value.count_ones();
        }
    }
    return squares_used as u64;
}

#[aoc(day14, part2)]
fn solve_part_2(input: &String) -> u64 {
    let mut grid_state = HashMap::<Point2D, bool>::new();
    for row in 0..=127 {
        let hash_input = format!("{}-{}", input, row.to_string());
        let knot_hash = calculate_knot_hash(&hash_input);
        for i in 0..16 {
            let value = knot_hash[i];
            for bit in 0..8 {
                let bit_value = (value >> (7 - bit)) & 0x01;
                // Calculate grid co-ordinates
                let x = (i * 8 + bit as usize) as i64;
                let y = row;
                let position = Point2D::new(x, y);
                // println!("x: {} --- y: {} --- bit_value: {} --- value: {}", x, y, bit_value, value);
                // Insert bit state into grid
                if bit_value == 1 {
                    grid_state.insert(position, true);
                } else {
                    grid_state.insert(position, false);
                }
            }
        }
    }
    // Determine regions present by finding what
    let mut regions = HashMap::<Point2D, u64>::new();
    let mut current_region = 0;
    for y in 0..=127 {
        for x in 0..=127 {
            // Check if current pos is used and not yet part of region
            let pos = Point2D::new(x, y);
            if *grid_state.get(&pos).unwrap() == true && !regions.contains_key(&pos) {
                process_regions(&pos, &grid_state, &mut regions, current_region);
                current_region += 1;
            }
        }
    }
    return current_region;
}

/// Determines the other used squares within the 128x128 grid that are adjacent (not diagonal) to
/// the given starting point.
fn process_regions(
    start_point: &Point2D,
    grid_state: &HashMap<Point2D, bool>,
    regions: &mut HashMap<Point2D, u64>,
    current_region: u64,
) {
    // Add start point to current region
    regions.insert(*start_point, current_region);
    // Get adjacent points to start point with non-negative co-ordinates within grid
    let mut adjacent_points = start_point.get_adjacent_points();
    adjacent_points.retain(|pos| {
        pos.get_x() >= 0 && pos.get_y() >= 0 && pos.get_x() <= 127 && pos.get_y() <= 127
    });
    // For each adjacent point, check if used
    for point in adjacent_points {
        if *grid_state.get(&point).unwrap() == true && !regions.contains_key(&point) {
            process_regions(&point, grid_state, regions, current_region);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d14_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day14.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(8190, result);
    }

    #[test]
    fn test_d14_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day14.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1134, result);
    }
}

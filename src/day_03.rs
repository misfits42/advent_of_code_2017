use super::utils::map::Point2D;

use std::collections::HashMap;

fn gen_simple_spiral(input: &str) -> (HashMap<Point2D, u64>, Point2D) {
    // Initialise variables required to generate spiral out to target value
    let mut spiral = HashMap::<Point2D, u64>::new();
    let mut current_value: u64 = 1;
    let target_value = input.parse::<u64>().unwrap();
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut delta_x: i64 = 1;
    let mut delta_y: i64 = 0;
    let mut last_pos = Point2D::new(0, 0);
    // Start generating the spiral
    loop {
        // Cease spiral generation if we have exceeded the target value
        if current_value > target_value {
            return (spiral, last_pos);
        }
        // Add current value to the spiral
        last_pos = Point2D::new(current_x, current_y);
        spiral.insert(last_pos, current_value);
        // Increment current value
        current_value += 1;
        // Calculate next position based on current position
        current_x += delta_x;
        current_y += delta_y;
        if current_x - 1 == current_y && current_x > 0 {
            delta_x = 0;
            delta_y = -1;
        } else if current_x == -1 * current_y && current_x > 0 {
            delta_x = -1;
            delta_y = 0;
        } else if current_x == current_y && current_x < 0 {
            delta_x = 0;
            delta_y = 1;
        } else if current_x == -1 * current_y && current_x < 0 {
            delta_x = 1;
            delta_y = 0;
        }
    }
}

fn gen_complex_spiral(input: &str) -> (HashMap<Point2D, u64>, Point2D) {
    let mut spiral = HashMap::<Point2D, u64>::new();
    let mut current_value = 1;
    let target_value = input.parse::<u64>().unwrap();
    let mut current_x: i64 = 0;
    let mut current_y: i64 = 0;
    let mut delta_x: i64 = 1;
    let mut delta_y: i64 = 0;
    let mut last_pos = Point2D::new(0, 0);
    // Generate the spiral
    loop {
        if current_value > target_value {
            return (spiral, last_pos);
        }
        // Calculate current value
        last_pos = Point2D::new(current_x, current_y);
        if current_x != 0 || current_y != 0 {
            let surr_points = last_pos.get_surrounding_points();
            let mut calc_val = 0;
            for point in surr_points {
                if spiral.contains_key(&point) {
                    calc_val += spiral.get(&point).unwrap();
                }
            }
            current_value = calc_val;
        }
        spiral.insert(last_pos, current_value);
        // Calculate next position based on current position
        current_x += delta_x;
        current_y += delta_y;
        if current_x - 1 == current_y && current_x > 0 {
            delta_x = 0;
            delta_y = -1;
        } else if current_x == -1 * current_y && current_x > 0 {
            delta_x = -1;
            delta_y = 0;
        } else if current_x == current_y && current_x < 0 {
            delta_x = 0;
            delta_y = 1;
        } else if current_x == -1 * current_y && current_x < 0 {
            delta_x = 1;
            delta_y = 0;
        }
    }
}

#[aoc_generator(day3)]
fn generate_input(input: &str) -> ((HashMap<Point2D, u64>, Point2D), (HashMap<Point2D, u64>, Point2D)) {
    let simple_spiral = gen_simple_spiral(input);
    let complex_spiral = gen_complex_spiral(input);

    return (simple_spiral, complex_spiral);
}

#[aoc(day3, part1)]
fn solve_part_1(input: &((HashMap<Point2D, u64>, Point2D), (HashMap<Point2D, u64>, Point2D))) -> i64 {
    let target_loc = input.0.1;
    let result = target_loc.get_manhattan_dist(Point2D::new(0, 0));
    return result;
}

#[aoc(day3, part2)]
fn solve_part_2(input: &((HashMap<Point2D, u64>, Point2D), (HashMap<Point2D, u64>, Point2D))) -> u64 {
    let target_loc = input.1.1;
    let result = *input.1.0.get(&target_loc).unwrap();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d03_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day3.txt").unwrap().trim());
        let result = solve_part_1(&input);
        assert_eq!(480, result);
    }

    #[test]
    fn test_d03_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day3.txt").unwrap().trim());
        let result = solve_part_2(&input);
        assert_eq!(349975, result);
    }
}

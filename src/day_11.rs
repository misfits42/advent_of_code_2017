use super::utils::map::Point3D;

enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest
}

impl HexDirection {
    fn from_string(input: &str) -> Option<HexDirection> {
        match input {
            "n" => return Some(HexDirection::North),
            "ne" => return Some(HexDirection::NorthEast),
            "se" => return Some(HexDirection::SouthEast),
            "s" => return Some(HexDirection::South),
            "sw" => return Some(HexDirection::SouthWest),
            "nw" => return Some(HexDirection::NorthWest),
            _ => None
        }
    }
}

#[aoc_generator(day11)]
fn generate_input(input: &str) -> Vec<HexDirection> {
    let mut directions = Vec::<HexDirection>::new();
    for dir in input.trim().split(",") {
        directions.push(HexDirection::from_string(dir).unwrap());
    }
    return directions;
}

#[aoc(day11, part1)]
fn solve_part_1(input: &Vec<HexDirection>) -> u64 {
    let mut current_loc = Point3D::new(0, 0, 0);
    // Process each movement
    for dir in input {
        current_loc = process_movement(&current_loc, dir);
    }
    // Determine min steps to reach the child process
    return current_loc.get_hex_min_dist_from_origin();
}

#[aoc(day11, part2)]
fn solve_part_2(input: &Vec<HexDirection>) -> u64 {

    let mut max_dist_from_origin = 0;
    let mut current_loc = Point3D::new(0, 0, 0);
    // Process each movement
    for dir in input {
        current_loc = process_movement(&current_loc, dir);
        // Calculate current distance from origin and check if this value is largest seen yet
        let dist_from_origin = current_loc.get_hex_min_dist_from_origin();
        if dist_from_origin > max_dist_from_origin {
            max_dist_from_origin = dist_from_origin;
        }
    }
    return max_dist_from_origin;
}

/// Moves the current location in the specified direction on the hexagonal tile grid.
fn process_movement(current_loc: &Point3D, direction: &HexDirection) -> Point3D {
    match direction {
        HexDirection::North => current_loc.move_point(0, 1, -1),
        HexDirection::NorthEast => current_loc.move_point(1, 0, -1),
        HexDirection::SouthEast => current_loc.move_point(1, -1, 0),
        HexDirection::South => current_loc.move_point(0, -1, 1),
        HexDirection::SouthWest => current_loc.move_point(-1, 0, 1),
        HexDirection::NorthWest => current_loc.move_point(-1, 1, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d11_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day11.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!(877, result);
    }

    #[test]
    fn test_d11_p2_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day11.txt").unwrap());
        let result = solve_part_2(&input);
        assert_eq!(1622, result);
    }
}

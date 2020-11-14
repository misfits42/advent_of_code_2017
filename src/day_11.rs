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
    let start_loc = Point3D::new(0, 0, 0);
    let mut current_loc = start_loc;
    // Process each movement
    for dir in input {
        current_loc = match dir {
            HexDirection::North => current_loc.move_point(0, 1, -1),
            HexDirection::NorthEast => current_loc.move_point(1, 0, -1),
            HexDirection::SouthEast => current_loc.move_point(1, -1, 0),
            HexDirection::South => current_loc.move_point(0, -1, 1),
            HexDirection::SouthWest => current_loc.move_point(-1, 0, 1),
            HexDirection::NorthWest => current_loc.move_point(-1, 1, 0)
        }
    }
    // Determine min steps to reach the child process
    return current_loc.get_hex_min_dist_from_origin();
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
}

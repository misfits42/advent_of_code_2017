use std::collections::HashMap;

use super::utils::map::{CardinalDirection, Point2D, Quadrant2D};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum TrackSegment {
    Vertical,
    Horizontal,
    Intersection,
    Continue
}

#[aoc_generator(day19)]
fn generate_input(input: &str) -> (HashMap<Point2D, TrackSegment>, HashMap<Point2D, char>) {
    // Keep a record of the letter locations and where track segments are located
    let mut letter_locs: HashMap<Point2D, char> = HashMap::new();
    let mut track_locs: HashMap<Point2D, TrackSegment> = HashMap::new();
    // Keep track of current location in map when reading input
    let mut x;
    let mut y = 0;
    for line in input.lines() {
        // Reset x-var at the start of each line
        x = 0;
        for c in line.chars() {
            // Check for end of line
            if c == '\r' || c == '\n' {
                break;
            }
            if c == '|' { // Vertical track segment
                let loc = Point2D::new(x, y);
                track_locs.insert(loc, TrackSegment::Vertical);
            } else if c == '-' { // Horizontal track segment
                let loc = Point2D::new(x, y);
                track_locs.insert(loc, TrackSegment::Horizontal);
            } else if c == '+' { // Track intersection
                let loc = Point2D::new(x, y);
                track_locs.insert(loc, TrackSegment::Intersection);
            } else if c.is_ascii_alphabetic() { // Letter location
                let loc = Point2D::new(x, y);
                letter_locs.insert(loc, c);
                track_locs.insert(loc, TrackSegment::Continue);
            }
            x += 1;
        }
        y += 1;
    }
    // Process track segments to replace track cross-overs with continue segments
    let mut replace_with_intersection: Vec<Point2D> = vec![];
    let mut replace_with_continue: Vec<Point2D> = vec![];
    for (loc, segment) in track_locs.iter() {
        // Check if segment is an intersection - continue if so
        if *segment == TrackSegment::Intersection {
            continue;
        } else if *segment == TrackSegment::Continue {
            let adj_points = loc.get_adjacent_points_in_quadrant(Quadrant2D::PosPos);
            let mut adjacent_count = 0;
            let mut adj_segment_locs: Vec<Point2D> = vec![];
            for point in adj_points {
                if track_locs.contains_key(&point) {
                    adjacent_count += 1;
                    adj_segment_locs.push(point);
                }
            }
            // Check if there is a continue section where an intersection segment should be
            if adjacent_count == 2 && adj_segment_locs[0].get_x() != adj_segment_locs[1].get_x() &&
                    adj_segment_locs[0].get_y() != adj_segment_locs[1].get_y() {
                replace_with_intersection.push(*loc);
            }
        } else {
            let adj_points = loc.get_adjacent_points_in_quadrant(Quadrant2D::PosPos);
            let mut adjacent_count = 0;
            for point in adj_points {
                if track_locs.contains_key(&point) {
                    adjacent_count += 1;
                }
            }
            if adjacent_count == 4 {
                replace_with_continue.push(*loc);
            }
        }
    }
    // Conduct replacements
    for loc in replace_with_continue {
        track_locs.insert(loc, TrackSegment::Continue);
    }
    for loc in replace_with_intersection {
        track_locs.insert(loc, TrackSegment::Intersection);
    }
    return (track_locs, letter_locs);
}

#[aoc(day19, part1)]
fn solve_part_1(input: &(HashMap<Point2D, TrackSegment>, HashMap<Point2D, char>)) -> String {
    // Get copy of track segment and letter locations
    let track_locs = input.0.clone();
    let letter_locs = input.1.clone();
    // Initialise starting variables
    let mut letters_seen = String::new();
    let mut direction = CardinalDirection::South;
    // Determine starting location
    let mut top_row = track_locs.keys().collect::<Vec<&Point2D>>();
    top_row.retain(|loc| loc.get_y() == 0);
    let mut current_loc = *top_row[0];
    loop {
        // Check if on a letter and record observation
        if let Some(letter) = letter_locs.get(&current_loc) {
            letters_seen.push(*letter);
        }
        // Check if on an intersection - change direction as needed
        if let Some(segment) = track_locs.get(&current_loc) {
            if *segment == TrackSegment::Intersection {
                // Determine locations to left and right of current location
                let (left, right) = calculate_left_and_right_locations(&current_loc, &direction);
                // Determine in which direction rotation should be made
                if track_locs.contains_key(&left) {
                    direction = direction.rotate_left();
                } else if track_locs.contains_key(&right) {
                    direction = direction.rotate_right();
                } else {
                    panic!("Day 19 Part 1 - should not get here!");
                }
            }
        }
        // Move to next position
        current_loc = current_loc.move_point_in_direction(&direction);
        // Check if track exists at current location - if not, we have reached the end of the map
        if !track_locs.contains_key(&current_loc) {
            break;
        }
    }
    return letters_seen;
}

/// Calculates the locations to the left and right of the given location, with the packet travelling
/// in the specified direction.
fn calculate_left_and_right_locations(loc: &Point2D, direction: &CardinalDirection) -> (Point2D, Point2D) {
    match direction {
        CardinalDirection::North => {
            let left = loc.move_point(-1, 0);
            let right = loc.move_point(1, 0);
            (left, right)
        },
        CardinalDirection::East => {
            let left = loc.move_point(0, -1);
            let right = loc.move_point(0, 1);
            (left, right)
        },
        CardinalDirection::South => {
            let left = loc.move_point(1, 0);
            let right = loc.move_point(-1, 0);
            (left, right)
        },
        CardinalDirection::West => {
            let left = loc.move_point(0, 1);
            let right = loc.move_point(0, -1);
            (left, right)
        }
    }
}

#[aoc(day19, part2)]
fn solve_part_2(input: &(HashMap<Point2D, TrackSegment>, HashMap<Point2D, char>)) -> String {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d19_p1_proper() {
        let input = generate_input(&std::fs::read_to_string("./input/2017/day19.txt").unwrap());
        let result = solve_part_1(&input);
        assert_eq!("QPRYCIOLU", result);
    }
}
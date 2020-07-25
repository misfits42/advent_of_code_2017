use super::utils::map::Point2D;
use super::utils::map::Quadrant2D;

use std::collections::HashMap;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[aoc_generator(day3)]
fn generate_input(input: &str) -> (HashMap<u64, Point2D>, u64) {
    let target = input.trim().parse::<u64>().unwrap();
    let mut spiral = HashMap::<u64, Point2D>::new();
    let mut current = 1;
    let mut current_loc = Point2D::new(0, 0);
    let mut dir = Direction::Right;
    let mut new_layer = false;
    while current <= target {
        // Add current value to the spiral
        spiral.insert(current, current_loc);
        // Update position and value
        match dir {
            Direction::Up => {
                current_loc = current_loc.move_point(0, -1);
            },
            Direction::Down => {
                current_loc = current_loc.move_point(0, 1);
            },
            Direction::Left => {
                current_loc = current_loc.move_point(-1, 0);
            },
            Direction::Right => {
                current_loc = current_loc.move_point(1, 0);
            }
        }
        if current == 1 {
            dir = Direction::Up;
        }
        current += 1;
        if new_layer {
            dir = Direction::Up;
            new_layer = false;
        }
        // Check if direction needs to be changed
        if current_loc.get_x().abs() == current_loc.get_y().abs() {
            let quadrant = current_loc.get_quadrant();
            match quadrant {
                Quadrant2D::PosNeg => {
                    dir = Direction::Left;
                },
                Quadrant2D::NegNeg => {
                    dir = Direction::Down;
                },
                Quadrant2D::NegPos => {
                    dir = Direction::Right;
                },
                Quadrant2D::PosPos => {
                    new_layer = true;
                    // dir = Direction::Up;
                },
                Quadrant2D::Origin => {
                    panic!("Should not get here!");
                }
            }
        }
    }
    return (spiral, target);
}

#[aoc(day3, part1)]
fn solve_part_1(input: &(HashMap<u64, Point2D>, u64)) -> i64 {
    let target_loc = input.0.get(&input.1).unwrap();
    let dist = target_loc.get_manhattan_dist(Point2D::new(0, 0));
    return dist;
}

#[aoc(day3, part2)]
fn solve_part_2(input: &(HashMap<u64, Point2D>, u64)) -> u64 {
    unimplemented!();
}

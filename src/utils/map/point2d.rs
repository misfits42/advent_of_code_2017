use std::cmp::Ordering;

use super::CardinalDirection;

pub enum Quadrant2D {
    PosPos, // x: +ve, y: -ve
    PosNeg, // x: +ve, y: -ve
    NegNeg, // x: -ve, y: -ve
    NegPos, // x: -ve, y: +ve
    Origin
}

/// Represents a single point with discrete co-ordinates on a two-dimensions Euclidean surface.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    x: i64,
    y: i64
}

impl Point2D {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn get_x(&self) -> i64 {
        return self.x;
    }

    pub fn get_y(&self) -> i64 {
        return self.y;
    }

    pub fn move_point(&self, delta_x: i64, delta_y: i64) -> Point2D {
        return Point2D::new(self.x + delta_x, self.y + delta_y);
    }

    /// Determines the resulting location by moving from the current point in the specified
    /// cardinal direction.
    pub fn move_point_in_direction(&self, direction: &CardinalDirection) -> Point2D {
        match direction {
            CardinalDirection::North => {
                return self.move_point(0, -1);
            },
            CardinalDirection::East => {
                return self.move_point(1, 0);
            },
            CardinalDirection::South => {
                return self.move_point(0, 1);
            },
            CardinalDirection::West => {
                return self.move_point(-1, 0);
            }
        }
    }

    pub fn get_quadrant(&self) -> Quadrant2D {
        if self.x == 0 && self.y == 0 {
            return Quadrant2D::Origin;
        } else if self.x > 0 && self.y < 0 {
            return Quadrant2D::PosNeg;
        } else if self.x < 0 && self.y < 0 {
            return Quadrant2D::NegNeg;
        } else if self.x < 0 && self.y > 0 {
            return Quadrant2D::NegPos;
        } else { // self.x > 0 && self.y > 0
            return Quadrant2D::PosPos;
        }
    }

    pub fn get_manhattan_dist(&self, other: Point2D) -> i64 {
        return (self.x - other.x).abs() + (self.y - other.y).abs();
    }

    pub fn get_adjacent_points(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        // Add points up, down, left, right first
        if self.x < i64::MAX {
            output.push(self.move_point(1, 0));
        }
        if self.x > i64::MIN {
            output.push(self.move_point(-1, 0));
        }
        if self.y < i64::MAX {
            output.push(self.move_point(0, 1));
        }
        if self.y > i64::MIN {
            output.push(self.move_point(0, -1));
        }
        return output;
    }

    /// Gets the points adjacent to the current point that remain within the quadrant with X and Y
    /// non-negative.
    fn get_adjacent_points_pospos_quadrant(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        if self.x > 0 {
            output.push(self.move_point(-1, 0));
        }
        if self.x < i64::MAX {
            output.push(self.move_point(1, 0));
        }
        if self.y > 0 {
            output.push(self.move_point(0, -1));
        }
        if self.y < i64::MAX {
            output.push(self.move_point(0, 1));
        }
        return output;
    }

    /// Gets the points adjacent to the current point that remain within the quadrant with X
    /// non-negative and Y non-positive.
    fn get_adjacent_points_posneg_quadrant(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        if self.x > 0 {
            output.push(self.move_point(-1, 0));
        }
        if self.x < i64::MAX {
            output.push(self.move_point(1, 0));
        }
        if self.y > i64::MIN {
            output.push(self.move_point(0, -1));
        }
        if self.y < 0 {
            output.push(self.move_point(0, 1));
        }
        return output;
    }

    /// Gets the points adjacent to the current point that remain within the quadrant with X
    /// non-positive and Y non-negative.
    fn get_adjacent_points_negpos_quadrant(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        if self.x > i64::MIN {
            output.push(self.move_point(-1, 0));
        }
        if self.x < 0 {
            output.push(self.move_point(1, 0));
        }
        if self.y > 0 {
            output.push(self.move_point(0, -1));
        }
        if self.y < i64::MAX {
            output.push(self.move_point(0, 1));
        }
        return output;
    }

    /// Gets the points adjacent to the current point that remain within quadrant with X and Y
    /// non-positive values.
    fn get_adjacent_points_negneg_quadrant(&self) -> Vec<Point2D> {
        let mut output: Vec<Point2D> = vec![];
        if self.x > i64::MIN {
            output.push(self.move_point(0, -1));
        }
        if self.x < 0 {
            output.push(self.move_point(1, 0));
        }
        if self.y > i64::MIN {
            output.push(self.move_point(0, -1));
        }
        if self.y < 0 {
            output.push(self.move_point(0, 1));
        }
        return output;
    }

    /// Gets the points adjacent to current point that remain within the specified 2D quadrant.
    pub fn get_adjacent_points_in_quadrant(&self, quadrant: Quadrant2D) -> Vec<Point2D> {
        match quadrant {
            Quadrant2D::Origin => return vec![],
            Quadrant2D::PosPos => {
                return self.get_adjacent_points_pospos_quadrant();
            },
            Quadrant2D::PosNeg => {
                return self.get_adjacent_points_posneg_quadrant();
            },
            Quadrant2D::NegPos => {
                return self.get_adjacent_points_negpos_quadrant();
            },
            Quadrant2D::NegNeg => {
                return self.get_adjacent_points_negneg_quadrant();
            }
        }
    }

    /// Calculates the 2D points that surround the current point. Bounds checking for i64 underflow
    /// and overflow is conducted - if either event would occur, the corresponding surrounding
    /// point is not added.
    pub fn get_surrounding_points(&self) -> Vec<Point2D> {
        // Add points up, down, left, right first
        let mut output: Vec<Point2D> = self.get_adjacent_points();
        // Add points in diagonal directions
        if self.x < i64::MAX && self.y < i64::MAX {
            output.push(self.move_point(1, 1));
        }
        if self.x < i64::MAX && self.y > i64::MIN {
            output.push(self.move_point(1, -1));
        }
        if self.x > i64::MIN && self.y < i64::MAX {
            output.push(self.move_point(-1, 1));
        }
        if self.x > i64::MIN && self.y > i64::MIN {
            output.push(self.move_point(-1, -1));
        }
        return output;
    }
}

impl Ord for Point2D {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            return Ordering::Less;
        } else if self.y == other.y {
            if self.x < other.x {
                return Ordering::Less;
            } else if self.x == other.x {
                return Ordering::Equal;
            } else {
                return Ordering::Greater;
            }
        } else {
            return Ordering::Greater;
        }
    }
}

impl PartialOrd for Point2D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

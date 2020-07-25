pub enum Quadrant2D {
    PosPos, // x: +ve, y: -ve
    PosNeg, // x: +ve, y: -ve
    NegNeg, // x: -ve, y: -ve
    NegPos, // x: -ve, y: +ve
    Origin
}

/// Represents a single point with discrete co-ordinates on a two-dimensions Euclidean surface.
#[derive(Copy, Clone, Hash)]
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
}
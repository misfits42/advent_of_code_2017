#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3D {
    x: i64,
    y: i64,
    z: i64
}

impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn get_x(&self) -> i64 {
        return self.x
    }

    pub fn get_y(&self) -> i64 {
        return self.y;
    }

    pub fn get_z(&self) -> i64 {
        return self.z;
    }

    pub fn move_point(&mut self, delta_x: i64, delta_y: i64, delta_z: i64) -> Point3D {
        return Point3D::new(self.x + delta_x, self.y + delta_y, self.z + delta_z);
    }

    /// Calculates the minimum distance from the origin when 3D point is used to model location on
    /// flat-oriented hexagonal tile grid.
    pub fn get_hex_min_dist_from_origin(&self) -> u64 {
        let coords = [self.x, self.y, self.z];
        return coords.iter().map(|x| x.abs()).max().unwrap() as u64;
    }
}
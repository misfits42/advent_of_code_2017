/// Represents a particle in three dimensions with a 3D location, velocity and acceleration.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Particle3D {
    position: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64)
}

impl Particle3D {
    pub fn new(pos: (i64, i64, i64), vel: (i64, i64, i64), accel: (i64, i64, i64)) -> Self {
        Self {
            position: pos,
            velocity: vel,
            acceleration: accel
        }
    }

    /// Returns the 3D point representing the location of the particle.
    pub fn get_position_3d(&self) -> (i64, i64, i64) {
        return self.position;
    }

    /// Returns the 3-tuple representing the three-dimensional velocity of the particle.
    pub fn get_velocity_3d(&self) -> (i64, i64, i64) {
        return self.velocity;
    }

    /// Returns the 3-tuple representing the three-dimensional velocity of the particle.
    pub fn get_acceleration_3d(&self) -> (i64, i64, i64) {
        return self.acceleration;
    }

    /// First updates the particle velocity by applying acceleration, then updates the position by
    /// applying velocity.
    pub fn update_pos_and_vel(&mut self) {
        // Apply velocity change from acceleration
        self.apply_velocity_change();
        // Apply position change from velocity
        self.apply_position_change();
    }

    /// Calculates the Manhattan distance of the particle from the specified position.
    pub fn get_manhattan_distance(&self, from: (i64, i64, i64)) -> i64 {
        let mut dist = 0;
        dist += (self.position.0 - from.0).abs();
        dist += (self.position.1 - from.1).abs();
        dist += (self.position.2 - from.2).abs();
        return dist;
    }

    /// Applies the change in velocity resulting from the acceleration of the particle.
    fn apply_velocity_change(&mut self) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;
        self.velocity.2 += self.acceleration.2;
    }

    /// Applies the change in position resulting from the velocity of the particle.
    fn apply_position_change(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }
}
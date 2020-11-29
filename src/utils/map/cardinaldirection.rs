/// Represents the cardinal map directions.
pub enum CardinalDirection {
    North,
    South,
    East,
    West
}

impl CardinalDirection {
    /// Determines the resulting cardinal direction when rotating to the left by 90 degress.
    pub fn rotate_left(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::West,
            CardinalDirection::East => return CardinalDirection::North,
            CardinalDirection::South => return CardinalDirection::East,
            CardinalDirection::West => return CardinalDirection::South
        }
    }

    /// Determines the resulting cardinal direction when rotating to the right by 90 degrees.
    pub fn rotate_right(&self) -> CardinalDirection {
        match self {
            CardinalDirection::North => return CardinalDirection::East,
            CardinalDirection::East => return CardinalDirection::South,
            CardinalDirection::South => return CardinalDirection::West,
            CardinalDirection::West => return CardinalDirection::North
        }
    }
}
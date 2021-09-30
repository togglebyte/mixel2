use std::ops::{AddAssign, Add};
use nightmare::{Vector, Position};

/// Coords in canvas space.
/// Coords places zero zero at the top left of the canvas.
#[derive(Debug, Copy, Clone)]
pub struct Coords(pub Position);

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Position::new(x, y))
    }

    pub fn zero() -> Self {
        Self(Position::zero())
    }

    pub fn to_translation(self, height: i32) -> Position<i32> {
        Position::new(self.0.x, height - self.0.y)
    }

    pub fn from_translation(translation: Position<i32>, height: i32) -> Self {
        Self(Position::new(translation.x, height - translation.y))
    }
}

impl From<Vector> for Coords {
    fn from(vec: Vector) -> Self {
        Self(vec.cast())
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Coords) {
        self.0.x += rhs.0.x;
        self.0.y += rhs.0.y;
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Coords) -> Self::Output {
        Coords::new(self.0.x + rhs.0.x, self.0.y + rhs.0.y)
    }
}

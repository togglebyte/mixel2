use std::ops::{AddAssign, Add};
use nightmare::{Vector, Position};

/// Coords in canvas space.
/// Coords places zero zero at the top left of the canvas.
#[derive(Debug, Copy, Clone)]
pub struct Coords(pub Position);

impl Coords {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Position::new(x, y))
    }

    pub fn zero() -> Self {
        Self(Position::zeros())
    }

    pub fn to_translation(self, height: f32) -> Position {
        Position::new(self.0.x, height - self.0.y)
    }

    pub fn from_translation(translation: Position, height: f32) -> Self {
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

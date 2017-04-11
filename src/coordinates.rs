use core::ops::{Add, Sub, AddAssign, SubAssign, Mul};


/// Represents an concrete Pixel on the screen.
/// Values are in `u16`, so no negative Values are possible!
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel16 {
    pub x: u16,
    pub y: u16,
}

/// Represents an abstract 2D coordinate
/// in an abstract coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord2D {
    pub x: f32,
    pub y: f32,
}

impl Add for Coord2D {
    type Output = Coord2D;

    fn add(self, other: Coord2D) -> Coord2D {
        Coord2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord2D {
    fn add_assign(&mut self, other: Coord2D) {
        *self = Coord2D {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Coord2D {
    type Output = Coord2D;

    fn sub(self, other: Coord2D) -> Coord2D {
        Coord2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Coord2D {
    fn sub_assign(&mut self, other: Coord2D) {
        *self = Coord2D {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul<f32> for Coord2D {
    type Output = Coord2D;

    fn mul(self, other: f32) -> Coord2D {
        Coord2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

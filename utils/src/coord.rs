use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

impl Sub for &Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Mul<i32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: i32) -> Self::Output {
        Coord::new(self.x*rhs, self.y*rhs)
    }
}
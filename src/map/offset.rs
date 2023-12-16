use std::ops::{Add, Neg};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
}

impl Offset {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub const fn x(x: isize) -> Self {
        Self { x, y: 0 }
    }

    pub const fn y(y: isize) -> Self {
        Self { x: 0, y }
    }
}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add<Self> for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[cfg(test)]
mod offset_tests {
    use super::*;

    #[test]
    fn test_offset_negation() {
        let offset = Offset { x: 1, y: -2 };

        let result = -offset;

        assert_eq!(result, Offset { x: -1, y: 2 });
    }

    #[test]
    fn test_offset_addition() {
        let offset1 = Offset { x: 1, y: 2 };
        let offset2 = Offset { x: 3, y: 4 };

        let result = offset1 + offset2;

        assert_eq!(result, Offset { x: 4, y: 6 });
    }
}

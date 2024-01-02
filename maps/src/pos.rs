use std::ops::{Add, AddAssign};

use serde::Deserialize;

use super::prelude::{Map, Offset};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn try_add(&self, rhs: &Offset) -> Option<Self> {
        Some(Self {
            x: self.x.checked_add_signed(rhs.x)?,
            y: self.y.checked_add_signed(rhs.y)?,
        })
    }

    pub fn try_add_consuming(mut self, rhs: Offset) -> Option<Self> {
        self.x = self.x.checked_add_signed(rhs.x)?;
        self.y = self.y.checked_add_signed(rhs.y)?;

        Some(self)
    }

    pub fn try_add_in_map(&self, map: &Map, rhs: &Offset) -> Option<Self> {
        let pos = self.try_add(rhs)?;

        if pos.x < map.width() && pos.y < map.height() {
            Some(pos)
        } else {
            None
        }
    }

    pub fn apply(&mut self, rhs: &Self) {
        self.x = rhs.x;
        self.y = rhs.y;
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_pos_addition() {
        let pos1 = Pos { x: 1, y: 2 };
        let pos2 = Pos { x: 3, y: 4 };

        let result = pos1 + pos2;

        assert_eq!(result, Pos { x: 4, y: 6 });
    }

    #[test]
    fn test_pos_add_assign() {
        let mut pos1 = Pos { x: 1, y: 2 };
        let pos2 = Pos { x: 3, y: 4 };

        pos1 += pos2;

        assert_eq!(pos1, Pos { x: 4, y: 6 });
    }

    #[test]
    fn test_pos_try_add() {
        let pos1 = Pos { x: 1, y: 3 };
        let offset = Offset::new(5, -1);

        let result = pos1.try_add_consuming(offset);

        assert_eq!(result, Some(Pos { x: 6, y: 2 }));
    }

    #[test]
    fn test_pos_try_add_1() {
        let pos1 = Pos {
            x: usize::MAX - 1,
            y: usize::MAX,
        };
        let offset = Offset::new(1, 0);

        let result = pos1.try_add_consuming(offset);

        assert_eq!(
            result,
            Some(Pos {
                x: usize::MAX,
                y: usize::MAX
            })
        );
    }

    #[test]
    fn test_pos_try_add_2() {
        let pos1 = Pos {
            x: usize::MAX - 1,
            y: usize::MAX,
        };
        let offset = Offset::new(1, 2);

        let result = pos1.try_add_consuming(offset);

        assert_eq!(result, None);
    }
}

use crate::W;

use super::prelude::{Map, Offset, Pos};

impl W<&Pos> {
    #[must_use]
    pub fn try_add(&self, rhs: &Offset) -> Option<Pos> {
        Some(Pos {
            x: self.0.x.checked_add_signed(rhs.x)?,
            y: self.0.y.checked_add_signed(rhs.y)?,
        })
    }

    #[must_use]
    pub fn try_add_in_map(&self, map: &Map, rhs: &Offset) -> Option<Pos> {
        let pos = self.try_add(rhs)?;

        if pos.x < map.width() && pos.y < map.height() {
            Some(pos)
        } else {
            None
        }
    }
}

impl W<&mut Pos> {
    pub fn apply(&mut self, rhs: &Pos) {
        self.0.x = rhs.x;
        self.0.y = rhs.y;
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
}

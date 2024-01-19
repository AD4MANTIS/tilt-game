use super::prelude::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    pub const fn to_offset(self) -> Offset {
        match self {
            Self::Top => Offset::NEG_Y,
            Self::Left => Offset::NEG_X,
            Self::Right => Offset::X,
            Self::Bottom => Offset::Y,
        }
    }
}
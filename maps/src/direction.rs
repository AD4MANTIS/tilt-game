use bevy_math::IVec2;

use crate::prelude::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub enum Horizontal {
    Top,
    Left,
    Right,
    Bottom,
}

impl Horizontal {
    #[must_use]
    pub const fn to_offset(self) -> Offset {
        match self {
            Self::Top => Offset::NEG_Y,
            Self::Left => Offset::NEG_X,
            Self::Right => Offset::X,
            Self::Bottom => Offset::Y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize)]
pub enum Diagonal {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Diagonal {
    #[must_use]
    pub const fn to_offset(self) -> IVec2 {
        match self {
            Self::TopLeft => IVec2::new(1, -1),
            Self::TopRight => IVec2::new(1, 1),
            Self::BottomLeft => IVec2::new(-1, 1),
            Self::BottomRight => IVec2::new(-1, -1),
        }
    }

    #[must_use]
    pub const fn horizontals(self) -> [Horizontal; 2] {
        match self {
            Self::TopLeft => [Horizontal::Top, Horizontal::Left],
            Self::TopRight => [Horizontal::Top, Horizontal::Right],
            Self::BottomLeft => [Horizontal::Bottom, Horizontal::Left],
            Self::BottomRight => [Horizontal::Bottom, Horizontal::Right],
        }
    }
}

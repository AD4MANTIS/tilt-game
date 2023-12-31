use std::fmt::Display;

use super::RockKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub rock: RockKind,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RockKind as Display>::fmt(&self.rock, f)
    }
}

impl From<RockKind> for Tile {
    fn from(value: RockKind) -> Self {
        Self { rock: value }
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'O' => RockKind::RoundRock,
            '#' => RockKind::SquareRock,
            '.' => RockKind::Empty,
            _ => return Err(()),
        }
        .into())
    }
}

use std::fmt::{Display, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub rock: RockKind,
}

impl Display for Tile {
    // TODO [`print_map`] needs custom logic in the future to print the whole map
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RockKind as Display>::fmt(&self.rock, f)
    }
}

impl From<RockKind> for Tile {
    fn from(value: RockKind) -> Self {
        Self { rock: value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rock {
    pub kind: RockKind,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RockKind as Display>::fmt(&self.kind, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RockKind {
    RoundRock = 'O' as isize,
    SquareRock = '#' as isize,
    Empty = '.' as isize,
}

impl Display for RockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(*self as u8 as char)
    }
}

impl Default for RockKind {
    fn default() -> Self {
        Self::Empty
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

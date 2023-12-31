use std::fmt::Display;

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
    RoundRock,
    SquareRock,
    Empty,
}

impl Display for RockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::RoundRock => "○",
            Self::SquareRock => "▨",
            Self::Empty => ".",
        })
    }
}

impl Default for RockKind {
    fn default() -> Self {
        Self::Empty
    }
}

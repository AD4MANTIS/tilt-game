use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rock {
    pub kind: RockKind,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RockKind as Display>::fmt(&self.kind, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, strum::EnumString, strum::Display)]
pub enum RockKind {
    #[strum(to_string = ".")]
    Empty,

    #[strum(to_string = "○")]
    RoundRock,

    #[strum(to_string = "▨")]
    SquareRock,
}

impl Default for RockKind {
    fn default() -> Self {
        Self::Empty
    }
}

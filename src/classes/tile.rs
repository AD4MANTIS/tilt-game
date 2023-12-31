use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

use super::RockKind;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Tile {
    pub rock: RockKind,
}

impl FromStr for Tile {
    type Err = <RockKind as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, strum::ParseError> {
        <RockKind as FromStr>::from_str(s).map(Self::from)
    }
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

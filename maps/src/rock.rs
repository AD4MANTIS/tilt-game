use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use serde::Deserialize;

use crate::prelude::Diagonal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rock {
    pub kind: RockKind,
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <RockKind as Display>::fmt(&self.kind, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum RockKind {
    Empty,
    // TODO: This should probably be removed from here
    // because it is not part of the map itself
    RoundRock,
    SquareRock,
    SingleReflect(Diagonal),
}

impl FromStr for RockKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "." => Self::Empty,
            "o" => Self::RoundRock,
            "#" => Self::SquareRock,
            "◢" => Self::SingleReflect(Diagonal::TopLeft),
            "◣" => Self::SingleReflect(Diagonal::TopRight),
            "◥" => Self::SingleReflect(Diagonal::BottomLeft),
            "◤" => Self::SingleReflect(Diagonal::BottomRight),
            _ => return Err(format!("{s} is not a RockKind")),
        })
    }
}

impl Display for RockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('•'),
            Self::RoundRock => f.write_char('○'),
            Self::SquareRock => f.write_char('▨'),
            Self::SingleReflect(direction) => f.write_str(match direction {
                Diagonal::TopLeft => "◢",
                Diagonal::TopRight => "◣",
                Diagonal::BottomLeft => "◥",
                Diagonal::BottomRight => "◤",
            }),
        }
    }
}

impl Default for RockKind {
    fn default() -> Self {
        Self::Empty
    }
}

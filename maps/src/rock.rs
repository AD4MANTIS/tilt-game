use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use serde::Deserialize;

use crate::prelude::Direction;

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

    RoundRock,

    SquareRock,

    SingleReflect(Direction),
}

impl FromStr for RockKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for RockKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => f.write_char('.'),
            Self::RoundRock => f.write_char('○'),
            Self::SquareRock => f.write_char('▨'),
            Self::SingleReflect(_) => todo!(),
        }
    }
}

impl Default for RockKind {
    fn default() -> Self {
        Self::Empty
    }
}

use crate::maps::prelude::Pos;
use serde::Deserialize;

// TODO use
#[derive(Debug, Clone, Deserialize)]
pub struct WinCondition {
    pub general: GeneralWinConditions,
    pub rocks: RockWinConditions,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RockWinConditions {
    Pos(Vec<Pos>),
    Exit(Vec<Pos>),
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct GeneralWinConditions {
    pub max_moves: Option<usize>,
}

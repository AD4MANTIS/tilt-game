use std::collections::HashSet;

use maps::prelude::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapState {
    pub rock_positions: HashSet<Pos>,
}

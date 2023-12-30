use std::fs;

use crate::{classes::WinCondition, maps::prelude::Map};

#[derive(Debug)]
pub struct MapData {
    pub map: Map,
    // TODO make not Optional in the future
    pub win: Option<WinCondition>,
}

pub fn load_map_data(level: u64) -> Option<MapData> {
    Some(MapData {
        map: load_map(level)?,
        win: match level {
            10 => fs::read_to_string(format!("../assets/level/{level}/win_conditions.ron"))
                .ok()
                .and_then(|conditions_raw| ron::from_str(&conditions_raw).ok()),
            _ => None,
        },
    })
}

fn load_map(level: u64) -> Option<Map> {
    Some(Map::from(match level {
        10 => include_str!("../assets/level/10/start.txt"),
        60 => include_str!("../assets/level/60/start.txt"),
        99 => include_str!("../assets/level/99/start.txt"),
        _ => return None,
    }))
}

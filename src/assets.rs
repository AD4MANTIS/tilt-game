use crate::maps::prelude::Map;

pub fn load_map(level: u64) -> Option<Map> {
    Some(Map::from(match level {
        10 => include_str!("../assets/level/10/start.txt"),
        60 => include_str!("../assets/level/60/start.txt"),
        99 => include_str!("../assets/level/99/start.txt"),
        _ => return None,
    }))
}

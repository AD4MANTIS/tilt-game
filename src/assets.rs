use crate::{
    classes::WinCondition,
    maps::prelude::{Map, MapData},
};

pub fn load_map_data(level: u64) -> Option<MapData> {
    Some(MapData {
        map: Map::from(match level {
            10 => include_str!("../assets/level/10/start.txt"),
            60 => include_str!("../assets/level/60/start.txt"),
            99 => include_str!("../assets/level/99/start.txt"),
            _ => return None,
        }),
        win: match level {
            10 => Some(
                ron::from_str::<WinCondition>(include_str!(
                    "../assets/level/10/win_conditions.ron"
                ))
                .unwrap(),
            ),
            _ => None,
        },
    })
}

use crate::{
    classes::{Level, WinCondition},
    maps::prelude::{Map, MapData},
};

pub fn load_map_data(level: Level) -> MapData {
    MapData {
        map: Map::from(match level {
            Level::Lv10 => include_str!("../assets/level/10/start.txt"),
            Level::Lv60 => include_str!("../assets/level/60/start.txt"),
            Level::Lv99 => include_str!("../assets/level/99/start.txt"),
        }),
        win: match level {
            Level::Lv10 => Some(
                ron::from_str::<WinCondition>(include_str!(
                    "../assets/level/10/win_conditions.ron"
                ))
                .unwrap(),
            ),
            _ => None,
        },
    }
}

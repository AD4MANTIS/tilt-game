use crate::{classes::Level, maps::prelude::MapData};

pub fn load_map_data(level: Level) -> MapData {
    let data = match level {
        Level::Lv10 => include_str!("../assets/levels/10.ron"),
        Level::Lv60 => include_str!("../assets/levels/60.ron"),
        Level::Lv99 => include_str!("../assets/levels/99.ron"),
    };

    let mut map_data = ron::from_str::<MapData>(data).unwrap();

    if map_data
        .map
        .rows
        .first()
        .map_or(false, |row| row.is_empty())
    {
        map_data.map.rows.remove(0);
    }

    map_data
}

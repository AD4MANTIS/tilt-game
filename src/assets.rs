use crate::{classes::Level, maps::prelude::MapData};

pub fn load_map_data(level: Level) -> MapData {
    let data = level.get_data();

    let mut map_data = ron::from_str::<MapData>(data).unwrap();

    if map_data
        .map
        .rows
        .first()
        .map_or(false, |row| row.is_empty())
    {
        map_data.map.rows.remove(0);
    }

    if map_data.map.rows.last().map_or(false, |row| row.is_empty()) {
        map_data.map.rows.pop();
    }

    map_data
}

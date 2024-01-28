use classes::Level;
use game_classes::MapData;
use maps::prelude::Map;

pub fn load_map_data(level: Level) -> MapData {
    let data = level.get_data();

    let mut map_data = ron::from_str::<MapData>(data);

    let mut map_data = map_data.unwrap_or_else(|_| panic!("Should load level {level:?}"));

    // if map_data.map.row_iter(0).next().is_none() {
    //     map_data.map.items.retain(|pos, _item| pos.y > 0);
    // }

    // let last_row = map_data
    //     .map
    //     .items
    //     .keys()
    //     .map(|pos| pos.y)
    //     .max()
    //     .unwrap_or_default();

    // if map_data.map.row_iter(last_row).next().is_none() {
    //     map_data.map.items.retain(|pos, _item| pos.y != last_row);
    // }

    map_data.map = Map::new(
        map_data
            .map
            .rows()
            .filter(|row| row.clone().next().is_some())
            .map(std::iter::Iterator::cloned),
    );

    map_data
}

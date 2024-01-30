use std::collections::HashSet;

use classes::Level;
use game_classes::{MapData, MapState};
use maps::prelude::{Map, RockKind};

pub fn load_map_data(level: Level) -> (MapData, MapState) {
    let data = level.get_data();

    let map_data = ron::from_str::<MapData>(data);

    let mut map_data = map_data.unwrap_or_else(|_| panic!("Should load level {level:?}"));

    let initial_state = prepare_map(&mut map_data);

    (map_data, initial_state)
}

pub fn prepare_map(map_data: &mut MapData) -> MapState {
    map_data.map = Map::new(
        map_data
            .map
            .rows()
            .filter(|row| row.clone().next().is_some())
            .map(std::iter::Iterator::cloned),
    );

    let mut initial_state = MapState {
        rock_positions: HashSet::new(),
    };

    let all_pos = map_data.map.all_pos().copied().collect::<Vec<_>>();
    for pos in all_pos {
        let Some(tile) = map_data.map.get_mut(&pos) else {
            continue;
        };

        if tile.rock == RockKind::RoundRock {
            tile.rock = RockKind::Empty;

            assert!(initial_state.rock_positions.insert(pos));
        }
    }

    initial_state
}

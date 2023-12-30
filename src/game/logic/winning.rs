use crate::{
    classes::{RockWinConditions, RoundResult},
    maps::prelude::MapData,
    rock::RockKind,
    Tile,
};

pub(super) fn check_result(map_data: &MapData) -> Option<RoundResult> {
    let Some(win) = &map_data.win else {
        return None;
    };

    match &win.rocks {
        RockWinConditions::Pos(pos) => match pos.iter().all(|pos| {
            map_data.map.get(pos)
                == Some(&Tile {
                    rock: RockKind::RoundRock,
                })
        }) {
            true => Some(RoundResult::Won),
            false => None,
        },
        RockWinConditions::Exit(_) => todo!(),
    }
}

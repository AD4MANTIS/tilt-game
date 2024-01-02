use classes::{LostReason, RockKind, RoundResult, Tile};
use game_classes::{MapData, RockWinConditions, RoundStats};

pub(super) fn check_result(map_data: &MapData, stats: &RoundStats) -> Option<RoundResult> {
    match &map_data.win.rocks {
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
    .or_else(|| {
        if let Some(max_moves) = map_data.win.general.max_moves {
            if stats.moves >= max_moves {
                return Some(RoundResult::Lost(LostReason::RoundsExceeded));
            }
        }

        None
    })
}

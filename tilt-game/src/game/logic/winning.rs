use std::collections::HashSet;

use classes::{LostReason, RoundResult, RoundStats};
use game_classes::{MapState, RockWinConditions, WinCondition};
use maps::prelude::*;

pub(super) fn check_result(
    win: &WinCondition,
    state: &MapState,
    round_stats: &RoundStats,
) -> Option<RoundResult> {
    match &win.rocks {
        RockWinConditions::Pos(pos) => {
            let rock_pos = state.rock_positions.iter().collect::<HashSet<&Pos>>();

            pos.iter()
                .all(|pos| rock_pos.contains(pos))
                .then_some(RoundResult::Won)
        }
        RockWinConditions::Exit(_) => todo!(),
    }
    .or_else(|| {
        if let Some(max_moves) = win.general.max_moves {
            if round_stats.moves >= max_moves {
                return Some(RoundResult::Lost(LostReason::RoundsExceeded));
            }
        }

        None
    })
}

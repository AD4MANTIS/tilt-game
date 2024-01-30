use classes::{LostReason, RoundResult, RoundStats};
use game_classes::{MapState, RockWinConditions, WinCondition};

pub(super) fn check_result(
    win: &WinCondition,
    state: &MapState,
    round_stats: &RoundStats,
) -> Option<RoundResult> {
    match &win.rocks {
        RockWinConditions::Pos(pos) => pos
            .iter()
            .all(|pos| state.rock_positions.contains(pos))
            .then_some(RoundResult::Won),
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

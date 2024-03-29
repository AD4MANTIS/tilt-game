use console::Term;

use classes::RoundStats;
use game_classes::{MapData, MapState, W};

use crate::{cli::Action, Result};

mod input;
mod tilt;
mod winning;

pub fn print_map(
    term: &Term,
    map_data: &MapData,
    state: &MapState,
    round_stats: &RoundStats,
) -> Result<()> {
    let display_map = format!("{:#?}", W((map_data, state)));
    let mut display_infos = String::new();

    if let Some(max_moves) = map_data.win.general.max_moves {
        display_infos += &format!("Move {} of {}", round_stats.moves, max_moves);
    }

    let mut parts = vec![display_map, display_infos];
    parts.retain(|part| !part.is_empty());
    let display = parts.join("\n");

    #[cfg(not(test))]
    term.clear_screen()?;

    term.write_line(&display)?;

    Ok(())
}

pub(super) fn play_level(
    term: &Term,
    map_data: &MapData,
    state: &mut MapState,
    round_stats: &mut RoundStats,
) -> Result<Action> {
    loop {
        let input = term.read_key()?;
        term.clear_line()?;

        if let Some(action) = input::handle_input(term, &input, map_data, state, round_stats)? {
            return Ok(action);
        }
    }
}

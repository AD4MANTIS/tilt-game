use crate::{classes::RoundStats, cli::Action, maps::prelude::*, Result};
use console::Term;

mod input;
mod tilt;
mod winning;

pub fn print_map(term: &Term, map_data: &MapData, stats: &RoundStats) -> Result<()> {
    let display_map = format!("{:#?}", map_data);
    let mut display_infos = String::new();

    if let Some(max_moves) = map_data.win.general.max_moves {
        display_infos += &format!("Move {} of {}", stats.moves, max_moves);
    }

    let mut parts = vec![display_map, display_infos];
    parts.retain(|part| !part.is_empty());
    let display = parts.join("\n");

    term.clear_screen()?;
    term.write_line(&display)?;

    Ok(())
}

pub(super) fn play_level(
    term: &Term,
    map_data: &mut MapData,
    stats: &mut RoundStats,
) -> Result<Action> {
    let mut rock_pos = tilt::get_all_round_rocks(&map_data.map);

    loop {
        let input = term.read_key()?;
        term.clear_line()?;

        if let Some(action) = input::handle_input(term, &input, map_data, stats, &mut rock_pos)? {
            return Ok(action);
        }
    }
}

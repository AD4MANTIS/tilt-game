use crate::{cli::Action, maps::prelude::*, Result};
use console::Term;

mod input;
mod tilt;
mod winning;

pub fn print_map(term: &Term, map: &MapData) -> Result<()> {
    let display_map = format!("{:#?}", map);
    term.clear_screen()?;
    term.write_str(&display_map)?;

    Ok(())
}

pub(super) fn play_level(term: &Term, map_data: &mut MapData) -> Result<Action> {
    let mut rock_pos = tilt::get_all_round_rocks(&map_data.map);

    loop {
        let input = term.read_key()?;
        term.clear_line()?;

        if let Some(action) = input::handle_input(term, &input, map_data, &mut rock_pos)? {
            return Ok(action);
        }
    }
}

use bevy::prelude::*;
use bevy::text::{TextSection, TextStyle};
use console::Term;

use game_classes::{MapData, RoundStats};

use crate::{cli::Action, Result};

mod input;
mod tilt;
mod winning;

pub fn generate_map_str<'a>(
    map_data: &'a MapData,
    stats: &'a RoundStats,
) -> impl Iterator<Item = TextSection> + 'a {
    let display_map = map_data.get_text_sections();
    let mut display_infos = None;

    if let Some(max_moves) = map_data.win.general.max_moves {
        display_infos = Some(TextSection::new(
            format!("Move {} of {}", stats.moves, max_moves),
            TextStyle::default(),
        ));
    }

    display_map
        .chain(display_infos)
        .filter(|part| !part.value.is_empty())
}

pub(super) fn play_level(
    term: &Term,
    map_data: &mut MapData,
    stats: &mut RoundStats,
    input: &Input<KeyCode>,
) -> Result<Option<Action>> {
    let mut rock_pos = tilt::get_all_round_rocks(&map_data.map);

    input::handle_input(term, input, map_data, stats, &mut rock_pos)
}

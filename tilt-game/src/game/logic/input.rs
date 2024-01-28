use console::{style, Key, Term};

use classes::RoundStats;
use game_classes::MapData;
use maps::prelude::*;

use crate::{
    cli::{parse_cmd, write_help_text, Action},
    Result,
};

use super::{tilt::tilt, winning::check_result};

pub(super) fn handle_input(
    term: &Term,
    input: &Key,
    map_data: &mut MapData,
    stats: &mut RoundStats,
    rock_pos: &mut [Pos],
) -> Result<Option<Action>> {
    let mut rotate_towards = None::<Horizontal>;

    match input {
        Key::Char('w') | Key::ArrowUp => {
            rotate_towards = Some(Horizontal::Top);
        }
        Key::Char('a') | Key::ArrowLeft => {
            rotate_towards = Some(Horizontal::Left);
        }
        Key::Char('s') | Key::ArrowDown => {
            rotate_towards = Some(Horizontal::Bottom);
        }
        Key::Char('d') | Key::ArrowRight => {
            rotate_towards = Some(Horizontal::Right);
        }
        Key::Char('?' | 'h') => {
            write_help_text(term)?;
            return Ok(None);
        }
        Key::Char('r') => {
            return Ok(Some(Action::RestartLevel));
        }
        Key::Char(':') => {
            term.write_str(&format!("{} ", style(":").cyan()))?;
            return Ok(parse_cmd(term)?);
        }
        Key::Escape => return Ok(Some(Action::Quit)),
        _ => {}
    };

    if let Some(rotate_towards) = rotate_towards {
        stats.moves += 1;

        tilt(term, rotate_towards, map_data, rock_pos, stats)?;

        if let Some(round_result) = check_result(map_data, stats) {
            return Ok(Some(Action::Result(round_result)));
        }
    }

    Ok(None)
}

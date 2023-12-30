use console::{style, Key, Term};

use crate::{
    cli::{parse_cmd, write_help_text, Action},
    maps::prelude::*,
    Result,
};

use super::{tilt::tilt, winning::check_result};

pub(super) fn handle_input(
    term: &Term,
    input: &Key,
    map_data: &mut MapData,
    rock_pos: &mut [Pos],
) -> Result<Option<Action>> {
    let mut rotate_towards = None::<Direction>;

    match input {
        Key::Char('w') | Key::ArrowUp => {
            rotate_towards = Some(Direction::Top);
        }
        Key::Char('a') | Key::ArrowLeft => {
            rotate_towards = Some(Direction::Left);
        }
        Key::Char('s') | Key::ArrowDown => {
            rotate_towards = Some(Direction::Bottom);
        }
        Key::Char('d') | Key::ArrowRight => {
            rotate_towards = Some(Direction::Right);
        }
        Key::Char('?' | 'h') => {
            write_help_text(term)?;
        }
        Key::Char('r') => {
            return Ok(Some(Action::RestartLevel));
        }
        Key::Char(':') => {
            term.write_str(&format!("{} ", style(":").cyan()))?;
            match parse_cmd(term)? {
                None => {}
                Some(action) => return Ok(Some(action)),
            };
        }
        Key::Escape => return Ok(Some(Action::Quit)),
        _ => {}
    };

    if let Some(rotate_towards) = rotate_towards {
        tilt(term, rotate_towards, map_data, rock_pos)?;

        if let Some(round_result) = check_result(map_data) {
            return Ok(Some(Action::Result(round_result)));
        }
    }

    Ok(None)
}

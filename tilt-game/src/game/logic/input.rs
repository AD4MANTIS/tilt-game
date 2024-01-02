use bevy::prelude::*;
use console::{style, Term};

use game_classes::{MapData, RoundStats};
use maps::prelude::Direction;
use maps::prelude::*;

use crate::{
    cli::{parse_cmd, write_help_text, Action},
    Result,
};

use super::{tilt::tilt, winning::check_result};

pub(super) fn handle_input(
    term: &Term,
    input: &Input<KeyCode>,
    map_data: &mut MapData,
    stats: &mut RoundStats,
    rock_pos: &mut [Pos],
) -> Result<Option<Action>> {
    let mut rotate_towards = None::<Direction>;

    for input in input.get_just_pressed() {
        match input {
            KeyCode::W | KeyCode::Up => {
                rotate_towards = Some(Direction::Top);
            }
            KeyCode::A | KeyCode::Left => {
                rotate_towards = Some(Direction::Left);
            }
            KeyCode::S | KeyCode::Down => {
                rotate_towards = Some(Direction::Bottom);
            }
            KeyCode::D | KeyCode::Right => {
                rotate_towards = Some(Direction::Right);
            }
            /*TODO: '?' | */
            KeyCode::H => {
                write_help_text(term)?;
                return Ok(None);
            }
            KeyCode::R => {
                return Ok(Some(Action::RestartLevel));
            }
            KeyCode::Colon => {
                term.write_str(&format!("{} ", style(":").cyan()))?;
                return Ok(parse_cmd(term)?);
            }
            KeyCode::Escape => return Ok(Some(Action::Quit)),
            _ => {}
        };
    }

    if let Some(rotate_towards) = rotate_towards {
        stats.moves += 1;

        tilt(rotate_towards, map_data, rock_pos)?;

        if let Some(round_result) = check_result(map_data, stats) {
            return Ok(Some(Action::Result(round_result)));
        }
    }

    Ok(None)
}

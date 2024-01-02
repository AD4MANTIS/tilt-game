use std::{str::FromStr, thread, time::Duration};

use console::{style, Key, Term};

use classes::{Level, RoundResult, RoundStats};
use game_classes::MapData;

use super::{init::init, logic::print_map};
use crate::{assets::load_map_data, cli::Action, Error, Result};

pub fn run() -> Result<()> {
    let term = Term::stdout();
    let term_err = Term::stderr();

    term.hide_cursor()?;

    init()?;

    let result = run_main_loop(&term, &term_err);

    term.show_cursor()?;

    result
}

fn run_main_loop(term: &Term, term_err: &Term) -> Result<()> {
    let mut current_level = Level::Lv1;
    let mut stats = RoundStats::default();
    let mut map_data = load_level(current_level, term, &mut stats)?;

    // When this loop ends the game quits
    loop {
        let result = super::logic::play_level(term, &mut map_data, &mut stats);

        let action = match result {
            Err(err) => {
                term_err.write_line(&format!("{}", err))?;
                continue;
            }
            Ok(action) => action,
        };

        match action {
            Action::LoadLevel(level) => {
                let Ok(level) = Level::from_str(&level) else {
                    term_err.write_line(&format!("{}", Error::LevelNotFound(level)))?;
                    continue;
                };

                map_data = load_level(level, term, &mut stats)?;
                current_level = level;
            }
            Action::Result(RoundResult::Won) => {
                term.write_line(&style("Level won!").on_green().to_string())?;

                thread::sleep(Duration::from_secs(1));

                term.write_str(r#"Continuing to next level... (press "r" to restart)"#)?;
                if term.read_key()? == Key::Char('r') {
                    map_data = reload_level(current_level, term, &mut stats)?;
                } else {
                    let next_level = current_level.get_next_level();
                    map_data = load_level(next_level, term, &mut stats)?;
                    current_level = next_level;
                }
            }
            Action::Result(RoundResult::Lost(_reason)) => {
                term.write_line(&style("You lost!").on_red().to_string())?;
                // TODO: print reason
                term.write_str("Restart level...")?;
                term.read_key()?;

                map_data = reload_level(current_level, term, &mut stats)?;
            }
            Action::RestartLevel => {
                map_data = reload_level(current_level, term, &mut stats)?;
            }
            Action::Quit => break,
        };
    }

    Ok(())
}

fn reload_level(current_level: Level, term: &Term, stats: &mut RoundStats) -> Result<MapData> {
    let map_data = load_map_data(current_level);
    *stats = RoundStats::default();

    print_map(term, &map_data, stats)?;

    Ok(map_data)
}

fn load_level(level: Level, term: &Term, stats: &mut RoundStats) -> Result<MapData> {
    let map_data = load_map_data(level);
    *stats = RoundStats::default();

    print_map(term, &map_data, stats)?;

    Ok(map_data)
}

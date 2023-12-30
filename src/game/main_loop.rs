use std::{thread, time::Duration};

use console::{style, Term};

use crate::{
    assets::load_map_data, classes::RoundResult, cli::Action, maps::prelude::MapData, Error, Result,
};

use super::{init::init, logic::print_map};

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
    let mut current_level = 10;
    let mut map_data = load_map_data(current_level).expect("starting level not found");
    print_map(term, &map_data)?;

    loop {
        let result = super::logic::play_level(term, &mut map_data);

        let action = match result {
            Err(err) => {
                term_err.write_line(&format!("{}", err))?;
                continue;
            }
            Ok(action) => action,
        };

        match action {
            Action::LoadLevel(level) => {
                if let Some(x) = load_level(level, term, term_err)? {
                    (current_level, map_data) = x;
                }
            }
            Action::Result(RoundResult::Won) => {
                term.write_line(&style("Level won!").on_green().to_string())?;

                thread::sleep(Duration::from_secs(1));

                term.write_str("Continuing to next level...")?;
                term.read_key()?;

                if let Some(x) = load_level(get_next_level(current_level), term, term_err)? {
                    (current_level, map_data) = x;
                }
            }
            Action::Result(RoundResult::Lost(_reason)) => {
                term.write_line(&style("You lost!").on_red().to_string())?;
                // TODO: print reason
                term.write_str("Restart level...")?;
                term.read_key()?;

                map_data = reload_level(current_level, term)?;
            }
            Action::RestartLevel => {
                map_data = reload_level(current_level, term)?;
            }
            Action::Quit => break,
        };
    }

    Ok(())
}

fn reload_level(current_level: u64, term: &Term) -> Result<MapData> {
    let map_data = load_map_data(current_level).expect("Current Level should be reloaded");

    print_map(term, &map_data)?;

    Ok(map_data)
}

const fn get_next_level(current_level: u64) -> u64 {
    match current_level {
        10 => 60,
        60 => 99,
        99 => 99,
        _ => 10,
    }
}

fn load_level(level: u64, term: &Term, term_err: &Term) -> Result<Option<(u64, MapData)>> {
    let Some(map_data) = load_map_data(level) else {
        term_err.write_line(&Error::LevelNotFound(level.to_string()).to_string())?;

        return Ok(None);
    };

    print_map(term, &map_data)?;

    Ok(Some((level, map_data)))
}

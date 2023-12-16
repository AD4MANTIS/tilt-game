use console::Term;

use crate::{assets::load_map, cli::Action, Error, Result};

use super::logic::print_map;

pub fn run_main_loop(term: &Term, term_err: &Term) -> Result<()> {
    let mut current_level = 10;
    let mut map = load_map(current_level).expect("starting level not found");
    print_map(term, &map)?;

    loop {
        let result = super::logic::play_level(term, &map);

        match result {
            Err(err) => term_err.write_line(&format!("{}", err))?,
            Ok(action) => {
                match action {
                    Action::LoadLevel(level) => {
                        let Some(m) = load_map(level) else {
                            term_err.write_line(&Error::Level404(level.to_string()).to_string())?;

                            continue;
                        };

                        current_level = level;
                        map = m;

                        print_map(term, &map)?;
                    }
                    Action::RestartLevel => {
                        map = load_map(current_level).unwrap();

                        print_map(term, &map)?;
                    }
                    Action::Quit => break,
                };
            }
        };
    }

    Ok(())
}

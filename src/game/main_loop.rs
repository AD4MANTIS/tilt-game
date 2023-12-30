use console::Term;

use crate::{assets::load_map_data, cli::Action, Error, Result};

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

        match result {
            Err(err) => term_err.write_line(&format!("{}", err))?,
            Ok(action) => {
                match action {
                    Action::LoadLevel(level) => {
                        let Some(m) = load_map_data(level) else {
                            term_err.write_line(&Error::Level404(level.to_string()).to_string())?;

                            continue;
                        };

                        current_level = level;
                        map_data = m;

                        print_map(term, &map_data)?;
                    }
                    Action::RestartLevel => {
                        map_data =
                            load_map_data(current_level).expect("Current Level should be reloaded");

                        print_map(term, &map_data)?;
                    }
                    Action::Quit => break,
                };
            }
        };
    }

    Ok(())
}

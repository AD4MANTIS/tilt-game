use console::Term;

use crate::{assets::load_map, cli::Actions, Error, Result};

pub fn run_main_loop(term: &Term) -> Result<()> {
    let mut current_level = 10;
    let mut map = load_map(current_level).expect("starting level not found");

    loop {
        let result = super::logic::spin_me_round(term, &map);

        match result {
            Err(err) => term.write_line(&format!("{}", err))?,
            Ok(None) => break,
            Ok(Some(action)) => {
                match action {
                    Actions::LoadLevel(level) => {
                        let Some(m) = load_map(level) else {
                            Term::stderr()
                                .write_line(&Error::Level404(level.to_string()).to_string())?;

                            continue;
                        };

                        current_level = level;
                        map = m;
                    }
                    Actions::RestartLevel => {
                        map = load_map(current_level).unwrap();
                    }
                };
            }
        };
    }

    Ok(())
}

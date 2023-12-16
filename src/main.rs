use console::Term;
use tilt_game::{game::main_loop::run_main_loop, Result};

fn main() -> Result<()> {
    let term = Term::stdout();
    let term_err = Term::stderr();

    term.hide_cursor()?;

    let result = run_main_loop(&term, &term_err);

    term.show_cursor()?;

    result
}

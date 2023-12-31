use std::io;

use clap::{Parser, Subcommand};
use console::{style, Term};

use classes::{RockKind, RoundResult};

use crate::game::setting;

type Result<T> = std::result::Result<T, CmdError>;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the about information
    About {},

    /// Load a level with the given number
    Level { level: String },

    /**
    List the settings
    Located at:
    - `~/.config/tilt-game`
    - `%appdata%/tilt-game/config`
    - `~/Library/Application Support/tilt-game`
    */
    Settings {
        #[arg(short, long)]
        list: bool,
    },
}

pub enum Action {
    LoadLevel(String),
    Result(RoundResult),
    RestartLevel,
    Quit,
}

#[derive(thiserror::Error, Debug)]
pub enum CmdError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Clap(#[from] clap::Error),
}

pub fn parse_cmd(term: &Term) -> Result<Option<Action>> {
    let cmd = term.read_line()?;

    if cmd.trim() == "?" {
        write_help_text(term)?;
        return Ok(None);
    }

    let cli = match Cli::try_parse_from(std::iter::once("").chain(cmd.split(' '))) {
        Ok(cli) => cli,
        Err(err) => {
            term.write_line(&format!("{}", err))?;
            return Ok(None);
        }
    };

    match cli.command {
        Commands::About {} => write_about_info(term)?,
        Commands::Level { level } => return Ok(Some(Action::LoadLevel(level))),
        Commands::Settings { list: _ } => {
            term.write_line(&format!("{:?}", setting()))?;
        }
    };

    Ok(None)
}

pub fn write_about_info(term: &Term) -> io::Result<()> {
    term.write_line(&format!(
        "AD4MANTIS\n{}",
        style("https://github.com/AD4MANTIS/tilt-game").underlined()
    ))
}

pub fn write_help_text(term: &Term) -> io::Result<()> {
    term.write_str(&format!(
        r"
Move the rocks around to win!

Rock Types
- empty space ({0})
- round, rolling rocks ({1})
- solid, non moving rocks ({2})

Controls:
[arrow keys] or wasd => move rocks / tilt platform
Escape => quit the game
h, ? => help
: => CLI
",
        RockKind::Empty,
        RockKind::RoundRock,
        RockKind::SquareRock
    ))
}

use std::io;

use clap::{Parser, Subcommand};
use console::{style, Term};
use strum::VariantNames;

use classes::{Level, RoundResult};
use maps::prelude::{Diagonal, RockKind};

use crate::game::setting;

type Result<T> = std::result::Result<T, CmdError>;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Shows the about Information
    About {},

    #[command(subcommand)]
    Level(LevelCommands),

    /**
    List the Settings
    Located at:
    - `~/.config/tilt-game`
    - `%appdata%/tilt-game/config`
    - `~/Library/Application Support/tilt-game`
    */
    Settings {
        #[arg(short, long)]
        list: bool,
    },

    /// Show the Games help
    #[command(name = "?", alias = "h")]
    Help,
}

#[derive(Subcommand, Debug)]
enum LevelCommands {
    /// List all Levels
    List,

    /// Load a Level with the given Name
    Load { level: String },
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

    let cli = match Cli::try_parse_from(std::iter::once("").chain(cmd.split(' '))) {
        Ok(cli) => cli,
        Err(err) => {
            term.write_line(&format!("{err}"))?;
            return Ok(None);
        }
    };

    match cli.command {
        Commands::About {} => write_about_info(term)?,
        Commands::Level(cmd) => match cmd {
            LevelCommands::List => term.write_line(Level::VARIANTS.join("\n").as_str())?,
            LevelCommands::Load { level } => return Ok(Some(Action::LoadLevel(level))),
        },
        Commands::Settings { list: _ } => {
            term.write_line(&format!("{:?}", setting()))?;
        }
        Commands::Help => write_help_text(term)?,
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
- empty Space ({0})
- round, rolling Rocks ({1})
- solid, non moving Rocks ({2})
- direction changing Corners ({3}, {4}, {5}, {6})

Controls:
Arrow or WASD Keys => move Rocks / tilt Platform
Escape => quit the game
h, ? => help
: => CLI
",
        RockKind::Empty,
        RockKind::RoundRock,
        RockKind::SquareRock,
        RockKind::SingleReflect(Diagonal::BottomLeft),
        RockKind::SingleReflect(Diagonal::BottomRight),
        RockKind::SingleReflect(Diagonal::TopLeft),
        RockKind::SingleReflect(Diagonal::TopRight),
    ))
}

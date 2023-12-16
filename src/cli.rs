use std::io;

use clap::{Parser, Subcommand};
use console::Term;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    About {},
    Level { level: u64 },
}

pub enum Actions {
    LoadLevel(u64),
    RestartLevel,
}

#[derive(thiserror::Error, Debug)]
pub enum CmdError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Clap(#[from] clap::Error),
}

pub fn parse_cmd(term: &Term) -> Result<Option<Actions>, CmdError> {
    let cmd = term.read_line()?;

    let cli = match Cli::try_parse_from(std::iter::once("").chain(cmd.split(' '))) {
        Ok(cli) => cli,
        Err(err) => {
            term.write_line(&format!("{}", err))?;
            return Ok(None);
        }
    };

    match cli.command {
        Commands::About {} => term.write_line("AD4MANTIS")?,
        Commands::Level { level } => return Ok(Some(Actions::LoadLevel(level))),
        // _ => term.write_line(&format!("{:#?}", cli))?,
    };

    Ok(None)
}

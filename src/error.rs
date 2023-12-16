use std::io;

use crate::cli::CmdError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Level {0} not found")]
    Level404(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Cmd(#[from] CmdError),
}

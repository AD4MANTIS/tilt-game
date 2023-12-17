use std::io;

use crate::{cli::CmdError, game::init::InitError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to initialize")]
    InitError(#[from] InitError),

    #[error("Level {0} not found")]
    Level404(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Cmd(#[from] CmdError),
}

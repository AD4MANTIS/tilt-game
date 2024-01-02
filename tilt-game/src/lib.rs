pub(crate) mod assets;
pub(crate) mod cli;
mod error;
pub(crate) mod game;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub use crate::game::main_loop::run;

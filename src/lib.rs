pub mod assets;
pub mod classes;
pub mod cli;
mod error;
pub mod game;
pub mod maps;
mod rock;

pub use rock::Tile;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub use crate::game::main_loop::run;

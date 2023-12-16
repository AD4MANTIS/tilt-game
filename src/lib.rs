pub mod assets;
pub mod cli;
mod error;
pub mod game;
pub mod map;
mod rock;

pub use rock::Rock;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) mod levels;
pub(crate) mod rock;
pub(crate) mod round_result;
pub(crate) mod round_stats;
pub(crate) mod tile;

pub use self::{
    levels::Level,
    rock::{Rock, RockKind},
    round_result::{LostReason, RoundResult},
    round_stats::RoundStats,
    tile::Tile,
};

pub struct W<T>(pub T);

pub trait EnumerateU32<T>: Iterator<Item = T> {
    fn enumerate_u32(self) -> impl Iterator<Item = (u32, T)>;
}

impl<T, U: Iterator<Item = T>> EnumerateU32<T> for U {
    fn enumerate_u32(self) -> impl Iterator<Item = (u32, T)> {
        self.enumerate()
            .flat_map(|(x, tile)| u32::try_from(x).map(|x| (x, tile)))
    }
}

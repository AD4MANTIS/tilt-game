pub(crate) mod levels;
pub(crate) mod round_result;
pub(crate) mod round_stats;

pub use self::{
    levels::Level,
    round_result::{LostReason, RoundResult},
    round_stats::RoundStats,
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

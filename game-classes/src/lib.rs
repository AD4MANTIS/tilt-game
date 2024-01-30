mod map_data;
mod round_state;
mod win_condition;

pub use self::{
    map_data::MapData,
    round_state::MapState,
    win_condition::{GeneralWinConditions, RockWinConditions, WinCondition},
};

pub struct W<T>(pub T);

use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct RoundStats {
    pub moves: usize,
}

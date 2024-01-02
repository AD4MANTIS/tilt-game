pub enum RoundResult {
    Won,
    Lost(LostReason),
}

pub enum LostReason {
    RoundsExceeded,
}

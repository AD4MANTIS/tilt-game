pub enum RoundResult {
    Won,
    // TODO
    #[allow(dead_code)]
    Lost(LostReason),
}

pub enum LostReason {
    RoundsExceeded,
}

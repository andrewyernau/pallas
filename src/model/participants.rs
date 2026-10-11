use crate::model::players::{PlayerId, Rating};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParticipantId(pub u32);

/// A Player inside one specific tournament.
/// Here, the data is immutable
pub struct Participant {
    id: ParticipantId,
    player_id: PlayerId,

    /// Tournament Pairing Number (TPN): initial ranking (rating, title, alphabetical)
    pairing_number: u16,
    rating_at_start: Option<Rating>, // None = unrated
    status: ParticipantStatus,
}

/// Whole-tournament state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParticipantStatus {
    Active,
    Withdrawn,
    Disqualified,
}

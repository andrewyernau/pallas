use crate::model::participants::ParticipantId;

pub type RoundNumber = u8;
pub type BoardNumber = u16;

pub struct Round {
    number: RoundNumber,
    status: RoundStatus,

    pairings: Vec<Pairing>, // Here we will allocate all player pairings i-j per round
    // Vec, not Option: at most one pairing-allocated bye, but several players can request a bye
    byes: Vec<ByeAssignment>,
}

/// Pending -> Paired (draft, arbiter reviews) -> Published (locked) -> InProgress -> Completed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundStatus {
    Pending,
    Paired,
    Published,
    InProgress,
    Completed,
}

/// Here we place the parings -> once the specific pairing is finished
/// we can add the result value
/// F.e: Board 10
/// player A vs player B
/// wins white (player A) -> GameResult::Played(WhiteWin)
/// How many points that is (1-0, 2-0...) is decided by the ScoringSystem, not stored here.
///
/// A pairing always has two players. An odd player out is not a pairing, it is a bye.
pub struct Pairing {
    board: BoardNumber,

    white: ParticipantId,
    black: ParticipantId,

    result: Option<GameResult>, // None = not finished yet
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOutcome {
    WhiteWin,
    Draw,
    BlackWin,
}

/// Forfeits give points but don't count for rating nor as a played game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Played(GameOutcome),

    WhiteForfeitWin,
    BlackForfeitWin,

    DoubleForfeit,
}

/// No opponent, no color, no rating. Its points come from ByePolicy (derived, not stored)
pub struct ByeAssignment {
    participant: ParticipantId,
    kind: ByeKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ByeKind {
    PairingAllocated, // odd number of players, usually worth a win
    HalfPoint,        // requested by the player
    ZeroPoint,        // requested by the player / absent
}

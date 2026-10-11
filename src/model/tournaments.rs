use crate::common::date::Date;
use crate::model::participants::Participant;
use crate::model::rounds::Round;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TournamentId(pub u32);

pub struct Tournament {
    id: TournamentId,
    name: String,
    location: Location,
    start_date: Date,
    end_date: Date,
    status: TournamentStatus,
    config: TournamentConfig, // editable in Draft, frozen from InProgress on

    participants: Vec<Participant>,
    rounds: Vec<Round>,
    // Standings, tie-breaks and rating changes are derived from rounds, never stored here
}

pub struct Location {
    // Example: Spain, Region de Murcia, Murcia, Calle Santo Cristo, 1, 30001
    country: String,
    region: String,
    city: String,
    locality: String,
    postcode: String,      // "08001" loses its leading 0 as a number; other countries use letters
    street_number: String, // "12B", "s/n"...
    street_name: String,
    place: String,
}

/// FSM:
/// Draft = Add elements, modifiable -> Ready
/// Ready = Validate data. Once Okay -> InProgress,
/// InProgress = From round 1 to round N -> InProgress || Finished
/// Finished = Completed last round -> outputs all data, Elo, winner... -> Archived
/// Archived = Don't show the game. Only reachable through specific consults
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TournamentStatus {
    Draft,
    Ready,
    InProgress,
    Finished,
    Archived,
}

pub struct TournamentConfig {
    number_of_rounds: u8, //no need more
    pairing_system: PairingSystem,
    scoring: ScoringSystem,
    tie_breaks: Vec<TieBreak>, // applied in order

    rating: RatingConfig,
    bye_policy: ByePolicy,
    time_control: TimeControl,
    acceleration: Option<AccelerationSystem>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingSystem {
    SimpleSwiss, // not FIDE-valid
    FideDutch,
}

/// Points are counted in half-points to avoid floats:
/// classic 1 - half - 0 => win: 2, draw: 1, loss: 0
pub type HalfPoints = u16;

pub struct ScoringSystem {
    win: HalfPoints,
    draw: HalfPoints,
    loss: HalfPoints,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TieBreak {
    Buchholz,
    BuchholzCut1,
    SonnebornBerger,
    DirectEncounter,
    NumberOfWins,
}

/// Rating uses 1 / half / 0, whatever the ScoringSystem really is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RatingConfig {
    NotRated,
    FideRated, // category taken from time_control
}

pub struct ByePolicy {
    pairing_allocated: HalfPoints,
    half_point: HalfPoints,
    max_requested_byes: u8,
}

/// Decides which rating (standard/rapid/blitz) is used for participants.
/// TODO: Instead maybe add  time information (1:30:00 + 0:10:00, etc... not only std rapid blitz)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameCategory {
    Standard,
    Rapid,
    Blitz,
}

pub struct TimeControl {
    category: GameCategory,
    base_minutes: u16,
    increment_seconds: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelerationSystem {
    Baku,
}

impl Tournament {
    pub fn validate(){
        unimplemented!();
    }

    pub fn start(){
        unimplemented!()
    }
}
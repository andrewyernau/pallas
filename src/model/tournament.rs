use crate::common::date::Date;
struct Tournament {
    name: String,
    location: Location,
    start_date: Date,
    end_date: Date,
    status: TournamentStatus,
    config: TournamentConfig,

}

struct Location {
    // Example: Spain, Region de Murcia, Murcia, Calle Santo Cristo, 1, 30001
    country: String,
    region: String,
    city: String,
    locality: String,
    postcode: u16,
    street_number: u16,
    street_name: String,
    place: String,
}

struct TournamentStatus{

}

struct TournamentConfig {
    number_of_rounds: u8, //no need more
    pairing_system: PairingSystem,
    scoring: ScoringSystem,
    tie_break: Vec<TieBreak>

    rating:RatingConfig,
    bye_policy: ByePolicy,
    time_control: TimeControl,
    acceleration: Option<AccelerationSystem>
}
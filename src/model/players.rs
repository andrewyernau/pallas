/// The person, independent of any tournament.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);

/// FIDE rating is declared as integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rating(pub u16);

pub struct Player {
    id: PlayerId,
    first_name: String,
    last_name: String,
    national_id: Option<String>, // DNI, passport...

    // Mandatory only for FIDE rated tournaments -> checked when going Draft -> Ready
    fide_id: Option<u32>,
    federation: Option<String>, // FIDE 3-letter code: ESP, FRA...

    // Current ratings
    standard_rating: Option<Rating>,
    rapid_rating: Option<Rating>,
    blitz_rating: Option<Rating>,
}

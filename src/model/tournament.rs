use crate::common::date::Date;
struct Tournament {
    name: String,
    location: Location,
    start_date: Date,
    end_date: Date,
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
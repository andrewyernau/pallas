struct Participant {
    participant_id: String,
    participant_name: String,
    participant_surname: String,
}

enum ParticipantStatus {
    Active,
    Withdrawn,
    Disqualified,
}
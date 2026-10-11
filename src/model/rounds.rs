struct Round {
    number: RoundNumber,
    status: RoundStatus,

    pairings: Vec<Pairing>, // Here we will allocate all player pairings i-j per round
    bye: Option<ByeAssignment>,
}

/// Here we place the parings -> once the specific pairing is finished
/// we can add the result value
/// F.e: Board 10
/// player A vs player B
/// both arrive so there is no bye
/// wins white (player A) -> game result 1-0 in the standard system (-> WhiteWin/Draw/BlackWin -> so
/// we can avoid overengineering for a result system)
struct Pairing {
    board: BoardNumber,

    // TODO: handle uneven pairings, might have no ID in a specific pairing
    white: ParticipantId,
    black: ParticipantId,

    result: Option<GameResult>,
}

enum GameResult {
    Played(PlayedResult),

    WhiteForfeitWin,
    BlackForfeitWin,

    DoubleForfeit,
}

struct ByeAssignment {
    participant: ParticipantId,
    kind: ByeKind,
    points: TournamentPoints,
}
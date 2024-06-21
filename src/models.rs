use crate::interface::{WithSeats, WithVotes};

#[derive(Debug)]
/// Represents a candidacy in an election.
pub struct Candidacy {
    votes: u32,
    seats: u16,
}

/// Represents a candidacy in an election.
///
/// A candidacy is defined by the number of votes received and the number of seats won.
/// Represents a candidacy in an election.
///
/// # Arguments
///
/// * `votes` - The number of votes received by the candidacy.
/// * `seats` - The number of seats won by the candidacy.
///
/// # Example
///
/// ```
/// use electosim::models::Candidacy;
///
/// let candidacy = Candidacy::new(1000, 3);
/// ```
impl Candidacy {
    pub fn new(votes: u32, seats: u16) -> Candidacy {
        Candidacy { votes, seats }
    }
}

impl WithVotes for Candidacy {
    fn get_votes(&self) -> u32 {
        self.votes
    }

    fn set_votes(&mut self, votes: u32) {
        self.votes = votes;
    }
}

impl WithSeats for Candidacy {
    fn get_seats(&self) -> u16 {
        self.seats
    }

    fn set_seats(&mut self, seats: u16) {
        self.seats = seats;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidacy() {
        let mut candidacy = Candidacy::new(1000, 3);
        assert_eq!(candidacy.get_votes(), 1000);
        assert_eq!(candidacy.get_seats(), 3);

        candidacy.set_votes(2000);
        candidacy.set_seats(5);

        assert_eq!(candidacy.get_votes(), 2000);
        assert_eq!(candidacy.get_seats(), 5);
    }
}

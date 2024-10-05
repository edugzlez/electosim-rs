/// Trait representing an entity that has votes.
pub trait WithVotes {
    /// Returns the number of votes.
    fn get_votes(&self) -> u32;

    /// Sets the number of votes.
    fn set_votes(&mut self, votes: u32);

    /// Increases the number of votes by a given amount.
    fn increase_votes(&mut self, n: i32) {
        let current_votes = self.get_votes();
        if n < 0 {
            self.set_votes(current_votes.saturating_sub(n.abs() as u32));
        } else {
            self.set_votes(current_votes + n as u32);
        }
    }
}

/// A trait for objects that have seats.
pub trait WithSeats {
    /// Returns the number of seats.
    fn get_seats(&self) -> u16;

    /// Sets the number of seats.
    fn set_seats(&mut self, seats: u16);

    /// Increases the number of seats by a given amount.
    fn increase_seats(&mut self, n: i16) {
        let current_seats = self.get_seats();
        if n < 0 {
            self.set_seats(current_seats.saturating_sub(n.abs() as u16));
        } else {
            self.set_seats(current_seats + n as u16);
        }
    }
}

impl<T> WithVotes for Box<&mut T>
where
    T: WithVotes,
{
    fn get_votes(&self) -> u32 {
        (**self).get_votes()
    }

    fn set_votes(&mut self, votes: u32) {
        (**self).set_votes(votes);
    }
}

impl<T> WithSeats for Box<&mut T>
where
    T: WithSeats,
{
    fn get_seats(&self) -> u16 {
        (**self).get_seats()
    }

    fn set_seats(&mut self, seats: u16) {
        (**self).set_seats(seats);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Candidate {
        votes: u32,
        seats: u16,
    }

    impl WithVotes for Candidate {
        fn get_votes(&self) -> u32 {
            self.votes
        }

        fn set_votes(&mut self, votes: u32) {
            self.votes = votes;
        }
    }

    impl WithSeats for Candidate {
        fn get_seats(&self) -> u16 {
            self.seats
        }

        fn set_seats(&mut self, seats: u16) {
            self.seats = seats;
        }
    }

    #[test]
    fn test_with_votes() {
        let mut candidate = Candidate {
            votes: 1000,
            seats: 0,
        };

        assert_eq!(candidate.get_votes(), 1000);
        candidate.set_votes(2000);
        assert_eq!(candidate.get_votes(), 2000);

        candidate.increase_votes(100);
        assert_eq!(candidate.get_votes(), 2100);

        candidate.increase_votes(-200);
        assert_eq!(candidate.get_votes(), 1900);
    }

    #[test]
    fn test_with_seats() {
        let mut candidate = Candidate {
            votes: 1000,
            seats: 50,
        };

        assert_eq!(candidate.get_seats(), 50);
        candidate.set_seats(100);
        assert_eq!(candidate.get_seats(), 100);

        candidate.increase_seats(10);
        assert_eq!(candidate.get_seats(), 110);

        candidate.increase_seats(-20);
        assert_eq!(candidate.get_seats(), 90);
    }

    #[test]
    fn test_with_box() {
        let mut candidate = Candidate {
            votes: 1000,
            seats: 50,
        };

        let mut boxed_candidate = Box::new(&mut candidate);

        assert_eq!(boxed_candidate.get_votes(), 1000);
        boxed_candidate.set_votes(2000);
        assert_eq!(boxed_candidate.get_votes(), 2000);

        boxed_candidate.set_seats(100);
        assert_eq!(boxed_candidate.get_seats(), 100);
    }
}

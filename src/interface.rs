/// Trait representing an entity that has votes.
pub trait WithVotes {
    /// Returns the number of votes.
    fn get_votes(&self) -> u32;

    /// Sets the number of votes.
    fn set_votes(&mut self, votes: u32);
}

/// A trait for objects that have seats.
pub trait WithSeats {
    /// Returns the number of seats.
    fn get_seats(&self) -> u16;

    /// Sets the number of seats.
    fn set_seats(&mut self, seats: u16);

    /// Increases the number of seats by a given amount.
    fn increase_seats(&mut self, n: u16) {
        let actual_seats = self.get_seats();
        self.set_seats(actual_seats + n);
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
    }

    impl WithVotes for Candidate {
        fn get_votes(&self) -> u32 {
            self.votes
        }

        fn set_votes(&mut self, votes: u32) {
            self.votes = votes;
        }
    }

    #[test]
    fn test_with_votes() {
        let mut candidate = Candidate { votes: 1000 };
        let mut boxed_candidate = Box::new(&mut candidate);

        assert_eq!(boxed_candidate.get_votes(), 1000);
        boxed_candidate.set_votes(2000);
        assert_eq!(candidate.get_votes(), 2000);
    }
}

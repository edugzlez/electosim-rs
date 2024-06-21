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

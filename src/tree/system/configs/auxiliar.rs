use crate::interface::{WithSeats, WithVotes};

pub struct Candidacy {
    pub id: u32,
    pub votes: u32,
    pub seats: u16,
}

impl WithSeats for Candidacy {
    fn get_seats(&self) -> u16 {
        self.seats
    }

    fn set_seats(&mut self, seats: u16) {
        self.seats = seats;
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

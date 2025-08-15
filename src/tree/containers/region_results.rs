use crate::tree::containers::single_result::SingleResult;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RegionResults {
    results: HashMap<u32, SingleResult>,
}

impl RegionResults {
    pub fn new() -> RegionResults {
        RegionResults {
            results: HashMap::new(),
        }
    }
}

impl RegionResults {
    pub fn get_votes(&self, id: u32) -> Option<u32> {
        match self.results.get(&id) {
            Some(result) => Some(result.votes),
            None => None,
        }
    }

    pub fn get_seats(&self, id: u32) -> Option<u32> {
        match self.results.get(&id) {
            Some(result) => Some(result.seats),
            None => None,
        }
    }

    pub fn increase_votes(&mut self, id: u32, votes: i32) {
        match self.results.get_mut(&id) {
            Some(result) => {
                result.votes = match votes {
                    x if x < 0 => {
                        if result.votes < x.abs() as u32 {
                            0
                        } else {
                            result.votes - x.abs() as u32
                        }
                    }
                    _ => result.votes + votes as u32,
                };
            }
            None => {
                self.results.insert(id, SingleResult::new(votes as u32, 0));
            }
        }
    }

    pub fn increase_seats(&mut self, id: u32, seats: i8) {
        match self.results.get_mut(&id) {
            Some(result) => {
                result.seats = match seats {
                    x if x < 0 => {
                        if result.seats < x.abs() as u32 {
                            0
                        } else {
                            result.seats - x.abs() as u32
                        }
                    }
                    _ => result.seats + seats as u32,
                };
            }
            None => {
                self.results.insert(id, SingleResult::new(0, seats as u32));
            }
        }
    }

    pub fn set_votes(&mut self, id: u32, votes: u32) {
        match self.results.get_mut(&id) {
            Some(result) => {
                result.votes = votes;
            }
            None => {
                self.results.insert(id, SingleResult::new(votes, 0));
            }
        }
    }

    pub fn set_seats(&mut self, id: u32, seats: u32) {
        match self.results.get_mut(&id) {
            Some(result) => {
                result.seats = seats;
            }
            None => {
                self.results.insert(id, SingleResult::new(0, seats));
            }
        }
    }

    pub fn iter_results(&self) -> impl Iterator<Item = (&u32, &SingleResult)> {
        self.results.iter()
    }

    pub fn remove_result(&mut self, id: u32) {
        self.results.remove(&id);
    }
}

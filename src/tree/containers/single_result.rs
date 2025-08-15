#[derive(Debug, Clone)]
pub struct SingleResult {
    pub votes: u32,
    pub seats: u32,
}
impl SingleResult {
    pub fn new(votes: u32, seats: u32) -> Self {
        SingleResult { votes, seats }
    }
}

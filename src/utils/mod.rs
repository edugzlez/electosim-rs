//! # Utils

use crate::interface::{WithSeats, WithVotes};

/// Clears the seats won by the candidates.
///
/// # Arguments
///
/// * `results` - A mutable reference to a vector of candidates.
///
/// # Example
///
/// ```rust
/// use electosim::utils::clear_results;
/// use electosim::interface::WithSeats;
/// use electosim::models::Candidacy;
///
/// fn main() {
///    let mut candidacies = vec![
///       Candidacy::new(2010, 9),
///       Candidacy::new(1018, 4),
///       Candidacy::new(86, 0),
///       Candidacy::new(77, 0),
///    ];
///
///   clear_results(&mut candidacies);
///
///   assert_eq!(candidacies[0].get_seats(), 0);
///   assert_eq!(candidacies[1].get_seats(), 0);
///   assert_eq!(candidacies[2].get_seats(), 0);
///   assert_eq!(candidacies[3].get_seats(), 0);
/// }
/// ```
pub fn clear_results<T>(results: &mut Vec<T>)
where
    T: WithSeats,
{
    results.iter_mut().for_each(|r| r.set_seats(0));
}

/// Computes the total number of votes.
///
/// # Arguments
///
/// * `results` - A reference to a vector of candidates.
///
/// # Example
///
/// ```rust
/// use electosim::utils::compute_total_votes;
/// use electosim::models::Candidacy;
///
/// fn main() {
///     let candidacies = vec![Candidacy::new(2010, 9), Candidacy::new(1018, 4)];
///     let total_votes = compute_total_votes(&candidacies);
///     println!("Total votes: {}", total_votes);
/// }
/// ```
pub fn compute_total_votes<T>(results: &Vec<T>) -> u32
where
    T: WithVotes,
{
    results.iter().map(|x| x.get_votes()).sum()
}

/// Computes the total number of seats.
///
/// # Arguments
///
/// * `results` - A reference to a vector of candidates.
///
/// # Example
///
/// ```rust
/// use electosim::utils::compute_total_seats;
/// use electosim::models::Candidacy;
///
/// fn main() {
///     let candidacies = vec![Candidacy::new(2010, 9), Candidacy::new(1018, 4)];
///     let total_seats = compute_total_seats(&candidacies);
///     println!("Total seats: {}", total_seats);
/// }
/// ```
#[allow(dead_code)]
pub fn compute_total_seats<T>(results: &Vec<T>) -> u16
where
    T: WithSeats,
{
    results.iter().map(|x| x.get_seats()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn listing_candidacy() {
        let candidacies = vec![
            crate::models::Candidacy::new(2010, 9),
            crate::models::Candidacy::new(1018, 4),
            crate::models::Candidacy::new(86, 0),
            crate::models::Candidacy::new(77, 0),
        ];

        assert_eq!(compute_total_votes(&candidacies), 3191);
        assert_eq!(compute_total_seats(&candidacies), 13);
    }

    #[test]
    fn clearing_results() {
        let mut candidacies = vec![
            crate::models::Candidacy::new(2010, 9),
            crate::models::Candidacy::new(1018, 4),
            crate::models::Candidacy::new(86, 0),
            crate::models::Candidacy::new(77, 0),
        ];

        clear_results(&mut candidacies);

        assert_eq!(candidacies[0].get_seats(), 0);
        assert_eq!(candidacies[1].get_seats(), 0);
        assert_eq!(candidacies[2].get_seats(), 0);
        assert_eq!(candidacies[3].get_seats(), 0);
    }
}

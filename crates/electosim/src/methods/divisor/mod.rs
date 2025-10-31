//! # Divisor methods
//! Each seat is assigned to the candidate with the highest result of the division of the number of votes by a divisor.
//! The divisor is a function that takes the number of seats won by the candidate and returns a float number.
//!
//! All divisor methods are implemented based on the [compute_divisor_method] function.

use crate::{
    interface::{WithSeats, WithVotes},
    utils::clear_results,
};

/// A factory for divisor methods.
///
///
/// # Arguments
///
/// * `results` - A mutable reference to a vector of candidates.
/// * `seats` - The number of seats available in the election.
/// * `divisor` - A function that takes the number of seats won by the candidate and returns a float number.
///
/// # Example (D'Hondt method)
///
/// ```rust
/// use electosim::methods::divisor::compute_divisor_method;
/// use electosim::models::Candidacy;
///
/// let mut candidacies = vec![
///    Candidacy::new(2010, 0),
///    Candidacy::new(1018, 0),
///    Candidacy::new(86, 0),
///    Candidacy::new(77, 0),
/// ];
///
/// compute_divisor_method(&mut candidacies, 13, |s| (s + 1) as f32).unwrap();
/// ```
pub fn compute_divisor_method<'a, T>(
    results: &'a mut Vec<T>,
    seats: u16,
    divisor: impl Fn(u16) -> f32,
) -> Result<(), &'a str>
where
    T: WithSeats + WithVotes,
{
    clear_results(results);

    for _ in 0..seats {
        let better_idx = results
            .iter()
            .map(|c| {
                let votes = c.get_votes() as f32;
                let seats = c.get_seats();
                let div = divisor(seats);

                votes / div
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b));

        match better_idx {
            Some((idx, _)) => results[idx].increase_seats(1),
            None => return Err("EMPTY_RESULTS"),
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn compute_dhondt<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| (s + 1) as f32)
}

#[allow(dead_code)]
pub fn compute_sainte_lague<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| (2 * s + 1) as f32)
}

#[allow(dead_code)]
pub fn compute_adams<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| s as f32)
}

#[allow(dead_code)]
pub fn compute_imperiali<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| (s + 2) as f32)
}

#[allow(dead_code)]
pub fn compute_huntington_hill<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| ((s * (s + 1)) as f32).sqrt())
}

#[allow(dead_code)]
pub fn compute_danish<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_divisor_method(results, seats, |s| (3 * s + 1) as f32)
}

#[allow(dead_code)]
pub fn compute_wta<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    clear_results(results);

    let better_idx = results
        .iter()
        .map(|c| c.get_votes())
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b));

    match better_idx {
        Some((idx, _)) => results[idx].set_seats(seats),
        None => return Err("EMPTY_RESULTS"),
    }

    Ok(())
}

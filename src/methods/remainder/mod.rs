//! # Remainder methods
//! In the remainder methods, the votes of each candidate are divided by the quota to obtain the number of seats each candidate will receive. The quota depends on the method used. The integer part of the division is the number of seats won by the candidate, and the remainder is used to assign the remaining seats.
//!
//! All remainder methods are implemented based on the [compute_remainder_method] function.

use crate::{
    interface::{WithSeats, WithVotes},
    utils::{clear_results, compute_total_votes},
};

use std::cmp::min;
struct RemainderResult {
    pub integer: u16,
    pub remainder: f32,
}

/// A factory for remainder methods.
///
/// # Arguments
///
/// * `results` - A mutable reference to a vector of candidates.
/// * `seats` - The number of seats available in the election.
/// * `quota_fn` - A function that takes the total number of votes and the number of seats available and returns a float number.
///
/// # Example (Hare method)
///
/// ```rust
/// use electosim::methods::remainder::compute_remainder_method;
/// use electosim::models::Candidacy;
///
/// let mut candidacies = vec![
///    Candidacy::new(2010, 0),
///    Candidacy::new(1018, 0),
///    Candidacy::new(86, 0),
///    Candidacy::new(77, 0),
/// ];
///
/// let quota_fn = |total_votes, seats| total_votes as f32 / seats as f32;
///
/// compute_remainder_method(&mut candidacies, 13, quota_fn).unwrap();
/// ```
pub fn compute_remainder_method<'a, T>(
    results: &'a mut Vec<T>,
    seats: u16,
    quota_fn: impl Fn(u32, u16) -> f32,
) -> Result<(), &'a str>
where
    T: WithSeats + WithVotes,
{
    clear_results(results);
    let total_votes = compute_total_votes(results);
    let quota = quota_fn(total_votes, seats);
    let mut seats_left = seats;

    let mut remainders: Vec<RemainderResult> = results
        .iter()
        .map(|r| {
            let votes = r.get_votes() as f32;
            let integer = (votes / quota).floor() as u16;
            let remainder = votes / quota - integer as f32;

            RemainderResult { integer, remainder }
        })
        .collect();

    remainders.iter().enumerate().for_each(|(idx, r)| {
        results[idx].set_seats(r.integer);
        seats_left -= min(r.integer, seats_left);
    });

    for _ in 0..seats_left {
        let better_idx = remainders
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.remainder.total_cmp(&b.remainder));

        match better_idx {
            Some((idx, _)) => {
                results[idx].increase_seats(1);
                remainders[idx].integer += 1;
                remainders[idx].remainder -= 1.0;
            }
            None => return Err("EMPTY_RESULTS"),
        }
    }

    Ok(())
}

pub fn compute_hare<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_remainder_method(results, seats, |total_votes, seats| {
        total_votes as f32 / seats as f32
    })
}

#[allow(dead_code)]
pub fn compute_droop<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_remainder_method(results, seats, |total_votes, seats| {
        (total_votes as f32 / (seats + 1) as f32).floor() + 1.0
    })
}

#[allow(dead_code)]
pub fn compute_hagenbach_bischoff<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_remainder_method(results, seats, |total_votes, seats| {
        total_votes as f32 / (seats + 1) as f32
    })
}

#[allow(dead_code)]
pub fn compute_imperiali_quotient<T>(results: &mut Vec<T>, seats: u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    compute_remainder_method(results, seats, |total_votes, seats| {
        (total_votes as f32 / (seats + 2) as f32).floor() + 1.0
    })
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    #[test]
    fn test_bug_hagenbach_seats() {
        let candidacies = vec![
            Candidacy::new(600, 9),
            Candidacy::new(31, 4),
            Candidacy::new(32, 0),
        ];

        let seats = 1000;
        let method = Method::HAGENBASCHBISCHOFF;
        let cutoff = 0.1;

        let mut ele = election![candidacies, seats, method, cutoff];

        ele.compute().expect("Can not compute method");
        ele.results.iter().for_each(|c| println!("{:?}", c));
    }
}

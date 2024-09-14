//! # ElectoSIM
//! ElectoSIM is a library that allows you to simulate simple elections using different methods.
//! ## Methods
//! The following methods are available:
//! - D'Hondt
//! - Webster/Sainte-Laguë
//! - Adams
//! - Imperiali
//! - Huntington-Hill
//! - Danish
//! - Hare-Niemeyer
//! - Hagenbach-Bischoff
//! - Imperiali - Quotient
//! - Droop
//! - Winner Takes All
//!
//! ## Usage
//!
//! ```rust
//! use electosim::*;
//!
//! fn main() {
//!    let mut election = election!(
//!         vec![
//!             candidacy!(2010, 9),
//!             candidacy!(1018, 4),
//!             candidacy!(86, 0),
//!             candidacy!(77, 0),
//!         ],
//!         13,
//!         Method::HAGENBASCHBISCHOFF,
//!         0.1
//!    );
//!
//!     election.compute().expect("Can not compute method");
//!     election.results.iter().for_each(|c| println!("{:?}", c));
//! }
//! ```
//!
//! The first statement in the `main` function creates a new [SimpleElection] with the candidates, the number of seats available, and the method to be used. The `compute` method is then called to compute the election results. Finally, the results are printed to the console.
//!
//! # `compute_` functions
//! A method is a function with type `fn(&mut Vec<T>, u16) -> Result<(), &str>` where `T` is a type that implements the [`WithVotes`][interface::WithVotes] and [`WithSeats`][interface::WithSeats] traits.
//! You can use the `compute_` functions directly if you want to compute the election results without using the [SimpleElection] struct. For example:
//! ```rust
//! use electosim::*;
//! use electosim::methods::divisor::compute_dhondt;
//!
//! fn main() {
//!    let mut candidacies = vec![
//!         candidacy!(2010, 0),
//!         candidacy!(1018, 0),
//!         candidacy!(86, 0),
//!         candidacy!(77, 0),
//!     ];
//!
//!    compute_dhondt(&mut candidacies, 13).unwrap();
//!
//!    candidacies.iter().for_each(|c| println!("{:?}", c));
//! }
//! ```
//!
//! There are some implementations of the `compute_` functions in the [methods::divisor] (ex: D'hondt) and [methods::remainder] (ex: Hare) modules.

pub mod interface;
pub mod macros;
pub mod methods;
pub mod models;
pub mod utils;

use interface::WithVotes;
use methods::get_method_function;
pub use methods::Method;
pub use models::Candidacy;
use utils::clear_results; // Add this line to import the SimpleElection struct

/// Represents a simple election.
pub struct SimpleElection {
    /// The results of the election.
    pub results: Vec<Candidacy>,
    /// The number of seats available in the election.
    pub seats: u16,
    /// The method used for the election.
    pub method: Method,
    /// Electoral cutoff
    pub cutoff: f32,
}

impl SimpleElection {
    /// Creates a new `SimpleElection` struct.
    pub fn new(results: Vec<Candidacy>, seats: u16, method: Method) -> Self {
        SimpleElection {
            results,
            seats,
            method,
            cutoff: 0.0,
        }
    }

    pub fn total_votes(&self) -> u32 {
        self.results.iter().map(|c| c.get_votes()).sum()
    }

    /// Computes the election results using the specified method.
    ///
    /// # Arguments
    ///
    /// * `self` - A mutable reference to the `SimpleElection` struct.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the computation is successful, otherwise returns an `Err` with an error message.
    pub fn compute(&mut self) -> Result<(), &str> {
        let fun = get_method_function(self.method);
        let total_votes = self.total_votes() as f32;
        let cutoff_votes = (total_votes * self.cutoff) as u32;
        // compute_with_cutoff(fun, &mut self.results, self.seats, cutoff_votes)
        clear_results(self.results.as_mut());

        let mut filtered_results = self
            .results
            .iter_mut()
            .filter(|c| c.get_votes() > cutoff_votes)
            .map(|c| Box::new(c))
            .collect::<Vec<_>>();

        fun(&mut filtered_results, self.seats).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use interface::WithSeats;

    use super::*;

    #[test]
    fn test_simple_election() {
        let mut election = election!(
            vec![
                candidacy!(2010, 9),
                candidacy!(1018, 4),
                candidacy!(86),
                candidacy!(77),
            ],
            13,
            Method::DHONDT
        );

        election.compute().unwrap();
        election.results.iter().for_each(|c| println!("{:?}", c));
    }

    #[test]
    fn test_load_new_election() {
        let _ = SimpleElection::new(
            vec![
                candidacy!(2010),
                candidacy!(1018),
                candidacy!(86),
                candidacy!(77),
            ],
            13,
            Method::DHONDT,
        );
    }

    #[test]
    fn test_with_cutoff() {
        let mut res = election!(
            vec![candidacy!(10, 0), candidacy!(1, 0),],
            13,
            Method::DHONDT,
            0.1
        );

        res.compute().unwrap();

        assert_eq!(res.results.len(), 2);
        assert_eq!(res.results[0].get_seats(), 13);
        assert_eq!(res.results[1].get_seats(), 0);
    }
}

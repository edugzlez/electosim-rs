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
//! use electosim::methods::Method;
//! use electosim::models::Candidacy;
//!
//! use electosim::SimpleElection;
//!
//! fn main() {
//!    let mut election = SimpleElection {
//!         results: vec![
//!             Candidacy::new(2010, 9),
//!             Candidacy::new(1018, 4),
//!             Candidacy::new(86, 0),
//!             Candidacy::new(77, 0),
//!         ],
//!         seats: 13,
//!         method: Method::HAGENBASCHBISCHOFF,
//!    };
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
//! use electosim::methods::divisor::compute_dhondt;
//! use electosim::models::Candidacy;
//!
//! fn main() {
//!    let mut candidacies = vec![
//!         Candidacy::new(2010, 0),
//!         Candidacy::new(1018, 0),
//!         Candidacy::new(86, 0),
//!         Candidacy::new(77, 0),
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
pub mod methods;
pub mod models;
pub mod utils;

use methods::{get_method_function, Method};
use models::Candidacy;

/// Represents a simple election.
pub struct SimpleElection {
    /// The results of the election.
    pub results: Vec<Candidacy>,
    /// The number of seats available in the election.
    pub seats: u16,
    /// The method used for the election.
    pub method: Method,
}

impl SimpleElection {
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
        fun(&mut self.results, self.seats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Candidacy;

    #[test]
    fn test_simple_election() {
        let mut election = SimpleElection {
            results: vec![
                Candidacy::new(2010, 9),
                Candidacy::new(1018, 4),
                Candidacy::new(86, 0),
                Candidacy::new(77, 0),
            ],
            seats: 13,
            method: Method::DHONDT,
        };

        election.compute().unwrap();
        election.results.iter().for_each(|c| println!("{:?}", c));
    }
}

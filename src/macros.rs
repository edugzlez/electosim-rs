/// Creates a new [SimpleElection] struct.
/// # Arguments
/// * `results` - A vector of [Candidacy] structs.
/// * `seats` - The number of seats available in the election.
/// * `method` - The method to be used for the election.
/// * `cutoff` - The electoral cutoff.
/// # Example
/// ```
/// use electosim::*;
///
/// let election = election!(
///     vec![
///         candidacy!(2010, 9),
///         candidacy!(1018, 4),
///         candidacy!(86, 0),
///         candidacy!(77, 0),
///     ],
///     13,
///     Method::HAGENBASCHBISCHOFF,
///     0.1
/// );
/// ```
#[macro_export]
macro_rules! election {
    ($results:expr) => {
        $crate::SimpleElection {
            results: $results,
            seats: 0,
            method: electosim::Method::DHONDT,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr) => {
        $crate::SimpleElection {
            results: $results,
            seats: $seats,
            method: electosim::Method::DHONDT,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr, $method:expr) => {
        $crate::SimpleElection {
            results: $results,
            seats: $seats,
            method: $method,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr, $method:expr, $coff:expr) => {
        $crate::SimpleElection {
            results: $results,
            seats: $seats,
            method: $method,
            cutoff: $coff,
        }
    };
}

#[macro_export]
macro_rules! candidacy {
    ($votes:expr) => {
        $crate::Candidacy::new($votes, 0)
    };
    ($votes:expr, $seats:expr) => {
        $crate::Candidacy::new($votes, $seats)
    };
}

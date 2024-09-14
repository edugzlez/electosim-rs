#[macro_export]
macro_rules! election {
    ($results:expr) => {
        electosim::SimpleElection {
            results: $results,
            seats: 0,
            method: electosim::Method::DHONDT,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr) => {
        electosim::SimpleElection {
            results: $results,
            seats: $seats,
            method: electosim::Method::DHONDT,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr, $method:expr) => {
        SimpleElection {
            results: $results,
            seats: $seats,
            method: $method,
            cutoff: 0.0,
        }
    };
    ($results:expr, $seats:expr, $method:expr, $coff:expr) => {
        SimpleElection {
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
        Candidacy::new($votes, 0)
    };
    ($votes:expr, $seats:expr) => {
        Candidacy::new($votes, $seats)
    };
}

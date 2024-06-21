pub mod divisor;
pub mod remainder;

use crate::interface::{WithSeats, WithVotes};

use divisor::*;
use remainder::*;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Method {
    DHONDT,
    HARE,
    DROOP,
    SAINTELAGUE,
    ADAMS,
    IMPERIALI,
    HUNTINGTONHILL,
    DANISH,
    WINNERTAKESALL,
    HAGENBASCHBISCHOFF,
    IMPERIALIQUOTIENT,
}

#[allow(unreachable_patterns)]
pub fn get_method_function<T>(method: Method) -> fn(&mut Vec<T>, u16) -> Result<(), &str>
where
    T: WithSeats + WithVotes,
{
    match method {
        Method::DHONDT => compute_dhondt,
        Method::HARE => compute_hare,
        Method::SAINTELAGUE => compute_sainte_lague,
        Method::ADAMS => compute_adams,
        Method::IMPERIALI => compute_imperiali,
        Method::HUNTINGTONHILL => compute_huntington_hill,
        Method::DANISH => compute_danish,
        Method::WINNERTAKESALL => compute_wta,
        Method::HAGENBASCHBISCHOFF => compute_hagenbach_bischoff,
        Method::IMPERIALIQUOTIENT => compute_imperiali_quotient,
        Method::DROOP => compute_droop,
    }
}

// test all methods
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Candidacy;

    const METHODS: [Method; 11] = [
        Method::DHONDT,
        Method::HARE,
        Method::DROOP,
        Method::SAINTELAGUE,
        Method::ADAMS,
        Method::IMPERIALI,
        Method::HUNTINGTONHILL,
        Method::DANISH,
        Method::WINNERTAKESALL,
        Method::HAGENBASCHBISCHOFF,
        Method::IMPERIALIQUOTIENT,
    ];

    #[test]
    fn test_all_methods() {
        let mut candidacies = vec![
            Candidacy::new(2010, 9),
            Candidacy::new(1018, 4),
            Candidacy::new(86, 0),
            Candidacy::new(77, 0),
        ];

        for method in METHODS.iter() {
            let f = get_method_function(*method);
            f(&mut candidacies, 13).unwrap();
        }
    }

    #[test]
    fn test_empty_results() {
        let mut candidacies: Vec<Candidacy> = vec![];

        for method in METHODS.iter() {
            let f = get_method_function(*method);
            assert_eq!(f(&mut candidacies, 13), Err("EMPTY_RESULTS"));
        }
    }
}

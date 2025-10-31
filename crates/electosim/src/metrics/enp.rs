/// Effective Number of Parties (ENP)
///
/// Calculates the Effective Number of Parties (ENP) using the Laakso-Taagepera method.
/// You can consider using votes or seats.
///
/// See more details at https://en.wikipedia.org/wiki/Effective_number_of_parties
///
pub fn enp_laakso_taagepera(results: &[f32]) -> f32 {
    let total: f32 = results.iter().sum();

    1f32 / results.iter().map(|c| (c / total).powi(2)).sum::<f32>()
}

/// Effective Number of Parties (ENP)
///
/// Calculates the Effective ENP using the Golosov method.
/// You can consider using votes or seats.
///
/// See more details at [Effective number of parties](https://en.wikipedia.org/wiki/Effective_number_of_parties)
///
pub fn enp_golosov(results: &[f32]) -> f32 {
    let total: f32 = results.iter().sum();
    let p_1 = results
        .iter()
        .map(|c| c / total)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    results
        .iter()
        .map(|c| c / total)
        .map(|p_i| p_i / (p_i + p_1.powi(2) - p_i.powi(2)))
        .sum::<f32>()
}

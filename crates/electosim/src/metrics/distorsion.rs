use crate::interface::{WithSeats, WithVotes};

/// Calculate the Loosemore-Hanby index for a given set of candidacies.
/// The Loosemore-Hanby index is a measure of electoral distortion, defined as the average absolute difference between the proportion of votes received by each candidate and the proportion of seats won by that candidate.
///
/// # Arguments
///
/// * `candidacies` - A slice of candidacies implementing the `WithSeats` and `WithVotes` traits.
///
/// # Returns
///
/// The Loosemore-Hanby index as a floating-point number.
///
/// See more at [Loosemore–Hanby index](https://en.wikipedia.org/wiki/Loosemore%E2%80%93Hanby_index)
///
pub fn loosemore_hanby_index<C: WithSeats + WithVotes>(candidacies: &[C]) -> f32 {
    let total_votes: f32 = candidacies.iter().map(|c| c.get_votes() as f32).sum();
    let total_seats: f32 = candidacies.iter().map(|c| c.get_seats() as f32).sum();

    let v = candidacies
        .iter()
        .map(|c| c.get_votes() as f32 / total_votes);
    let s = candidacies
        .iter()
        .map(|c| c.get_seats() as f32 / total_seats);

    v.zip(s).map(|(v_i, s_i)| (v_i - s_i).abs()).sum::<f32>() / 2f32
}

/// The Rose index is a measure of electoral distortion, defined as the average absolute difference between the proportion of votes received by each candidate and the proportion of seats won by that candidate.
///
/// # Arguments
///
/// * `candidacies` - A slice of candidacies implementing the `WithSeats` and `WithVotes` traits.
///
/// # Returns
///
/// The Rose index as a floating-point number.
///
/// See more at [Loosemore–Hanby index](https://en.wikipedia.org/wiki/Loosemore%E2%80%93Hanby_index)
///
pub fn rose_index<C: WithSeats + WithVotes>(candidacies: &[C]) -> f32 {
    1f32 - loosemore_hanby_index(candidacies)
}

/// The Sainte-Lague index is a measure of electoral distortion, defined as the average squared difference between the proportion of votes received by each candidate and the proportion of seats won by that candidate.
///
/// # Arguments
///
/// * `candidacies` - A slice of candidacies implementing the `WithSeats` and `WithVotes` traits.
///
/// # Returns
///
/// The Sainte-Lague index as a floating-point number.
///
/// See more at [Sainte-Lague index](https://en.wikipedia.org/wiki/Sainte-Lagu%C3%AB_Index) and [Sainte-Lague method](https://en.wikipedia.org/wiki/Sainte-Lagu%C3%AB_method)
///
pub fn sainte_lague_index<C: WithSeats + WithVotes>(candidacies: &[C]) -> f32 {
    let total_votes: f32 = candidacies.iter().map(|c| c.get_votes() as f32).sum();
    let total_seats: f32 = candidacies.iter().map(|c| c.get_seats() as f32).sum();

    let v = candidacies
        .iter()
        .map(|c| c.get_votes() as f32 / total_votes);
    let s = candidacies
        .iter()
        .map(|c| c.get_seats() as f32 / total_seats);

    v.zip(s)
        .map(|(v_i, s_i)| (s_i - v_i).powi(2) / v_i)
        .sum::<f32>()
        / 2f32
}

/// The Gallagher index is a measure of electoral distortion, defined as the square root of the average squared difference between the proportion of votes received by each candidate and the proportion of seats won by that candidate.
///
/// # Arguments
///
/// * `candidacies` - A slice of candidacies implementing the `WithSeats` and `WithVotes` traits.
///
/// # Returns
///
/// The Gallagher index as a floating-point number.
///
/// See more at [Gallagher index](https://en.wikipedia.org/wiki/Gallagher_index)
///
pub fn gallagher_index<C: WithSeats + WithVotes>(candidacies: &[C]) -> f32 {
    let total_votes: f32 = candidacies.iter().map(|c| c.get_votes() as f32).sum();
    let total_seats: f32 = candidacies.iter().map(|c| c.get_seats() as f32).sum();

    let v = candidacies
        .iter()
        .map(|c| c.get_votes() as f32 / total_votes);
    let s = candidacies
        .iter()
        .map(|c| c.get_seats() as f32 / total_seats);

    (v.zip(s).map(|(v_i, s_i)| (s_i - v_i).powi(2)).sum::<f32>() / 2f32).sqrt()
}

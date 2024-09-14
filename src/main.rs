use electosim::*;

fn main() {
    let candidacies = vec![candidacy!(600, 9), candidacy!(31, 4), candidacy!(32, 0)];

    let seats = 1000;
    let method = Method::HAGENBASCHBISCHOFF;
    let cutoff = 0.1;

    let mut ele = election!(candidacies, seats, method, cutoff);

    ele.compute().expect("Can not compute method");
    ele.results.iter().for_each(|c| println!("{:?}", c));
}

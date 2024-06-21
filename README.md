# ElectoSIM ~ Rust

 [![crates.io](https://img.shields.io/crates/v/electosim.svg)](https://crates.io/crates/electosim) [![docs.rs](https://docs.rs/electosim/badge.svg)](https://docs.rs/electosim) [![codecov](https://codecov.io/gh/edugzlez/electosim-rs/graph/badge.svg?token=PZ76N09B8B)](https://codecov.io/gh/edugzlez/electosim-rs)

ElectoSIM is a Rust library for simulating elections using different voting systems.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
electosim = "0.1.0"
```

or add it directly from crates.io:

```sh
cargo add electosim
```

## Usage

### With SimpleElection

```rust
use electosim::methods::Method;
use electosim::models::Candidacy;

use electosim::SimpleElection;

fn main() {
   let mut election = SimpleElection {
        results: vec![
            Candidacy::new(2010, 9),
            Candidacy::new(1018, 4),
            Candidacy::new(86, 0),
            Candidacy::new(77, 0),
        ],
        seats: 13,
        method: Method::HAGENBASCHBISCHOFF,
   };

    election.compute().expect("Can not compute method");
    election.results.iter().for_each(|c| println!("{:?}", c));
}
```

### Directly with the compute_ method

```rust
use electosim::methods::divisor::compute_dhondt;
use electosim::models::Candidacy;

fn main() {
   let mut candidacies = vec![
        Candidacy::new(2010, 0),
        Candidacy::new(1018, 0),
        Candidacy::new(86, 0),
        Candidacy::new(77, 0),
    ];
   compute_dhondt(&mut candidacies, 13).unwrap();
   candidacies.iter().for_each(|c| println!("{:?}", c));
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

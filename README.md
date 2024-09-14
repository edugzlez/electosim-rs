# ElectoSIM ~ Rust

 [![crates.io](https://img.shields.io/crates/v/electosim.svg)](https://crates.io/crates/electosim) [![docs.rs](https://docs.rs/electosim/badge.svg)](https://docs.rs/electosim) [![codecov](https://codecov.io/gh/edugzlez/electosim-rs/graph/badge.svg?token=PZ76N09B8B)](https://codecov.io/gh/edugzlez/electosim-rs)

ElectoSIM is a Rust library for simulating elections using different voting systems.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
electosim = "0.2.0"
```

or add it directly from crates.io:

```sh
cargo add electosim
```

## Usage

### Using macro `election!`

```rust
use electosim::*;

fn main() {
    let candidacies = vec![
        candidacy!(2010, 9),
        candidacy!(1018, 4),
        candidacy!(86, 0),
        candidacy!(77, 0),
    ];

    let seats = 13;
    let method = Method::HAGENBASCHBISCHOFF;
    let cutoff = 0.1;

    let mut ele = election![candidacies, seats, method, cutoff];

    ele.compute().expect("Can not compute method");
    ele.results.iter().for_each(|c| println!("{:?}", c));
}
```

### Directly with the compute_ method

```rust
use electosim::*;
use electosim::methods::divisor::compute_dhondt;

fn main() {
   let mut candidacies = vec![
        candidacy!(2010, 0),
        candidacy!(1018, 0),
        candidacy!(86, 0),
        candidacy!(77, 0),
    ];
   compute_dhondt(&mut candidacies, 13).unwrap();
   candidacies.iter().for_each(|c| println!("{:?}", c));
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

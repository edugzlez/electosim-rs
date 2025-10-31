# ElectoSIM ~ Rust

ElectoSIM is a Rust library for simulating elections using different voting systems.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
electosim = "0.3.0"
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

### Directly with the compute\_ method

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

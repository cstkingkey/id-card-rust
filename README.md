Fork of https://github.com/Ethan1225/id-card-rust

# cn-id-card

Chinese identification card number verification, including region code, birthday, and check code.

## License

APL 2.0, see LICENSE file.

## Installation

In Cargo.toml add

```toml
[dependencies]
cn-id-card="0.1"
```

Then re-run `cargo build`. That fetches the dependencies and builds
the code.

## Usage

```rust
extern crate cn_id_card;

fn main() {
    println!("{}", cn_id_card::validate("440524188001010014", true));//outputs: true
}

```

## Development

Test changes before commitment.(Install nightly if it haven't been installed: rustup install nightly)
```shell
cargo clean
cargo fmt --all -- --check
cargo build
cargo +nightly bench
cargo test

```
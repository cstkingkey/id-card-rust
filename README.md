# id-card

Chinese identification card number verification, including region code, birthday, and check code.

## License

APL 2.0, see LICENSE file.

## Installation

In Cargo.toml add

```toml
[dependencies]
id-card="0.1"
```

Then re-run `cargo build`. That fetches the dependencies and builds
the code.

## Usage

```rust
extern crate id_card;

fn main() {
    println!("{}", id_card::validate("440524188001010014"));//outputs: true
}

```

## Development

Test changes before comitting.

```shell
cargo clean
cargo fmt --all -- --check
cargo build
cargo test
```
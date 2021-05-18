# edit-distance

Chinese identification card number verification, including region code, birthday, and check code.

## License

APL 2.0, see LICENSE file.

## Installation

In Cargo.toml add

```toml
[dependencies]
edit-distance = "2.1.0"
```

Then re-run `cargo build`. That fetches the dependencies and builds
the code.

## Usage

```rust
extern crate edit_distance;

edit_distance("kitten", "sitting"); // => 3
```

## Development

Test changes before comitting.

```shell
cargo clean
cargo fmt --all -- --check
cargo build
cargo test
```
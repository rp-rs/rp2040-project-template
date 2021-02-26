# Raspberry pico RP2040 rust test!

## Requirements

- Recent nightly Rust
- `probe-run-rp` installed.
- A CMSIS-DAP probe. (JLink probes sort of work but are very unstable. Other probes won't work at all)

`probe-run-rp` is a version of `probe-run` using a `probe-rs` fork with support for the RP2040 chip. To install it use the following command.

    cargo install --git https://github.com/rp-rs/probe-run --branch main

Note that this installs the binary with name `probe-run-rp`, so you can still have the original `probe-run` installed in parallel. This is important because `probe-run-rp` ONLY works with the RP2040 chip.


## Running

Just do `cargo run` :)
  
## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

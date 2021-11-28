# Project template for rp2040-hal

This template is intended as a starting point for developing your own firmware based on the rp2040-hal.

It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

`probe-run` is configured as the default runner, so you can start your program as easy as
```
DEFMT_LOG=trace cargo run --release
```

## Requirements
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- probe-run. Upstream support for RP2040 was added with version 0.3.1.

- A CMSIS-DAP probe. (JLink probes sort of work but are very unstable. Other probes won't work at all)

  You can use a second Pico as a CMSIS-DAP debug probe by installing the following firmware on it:
  https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2

  More details on supported debug probes can be found in [debug_probes.md](debug_probes.md)

## Installation of development dependencies
```
rustup target install thumbv6m-none-eabi
cargo install probe-run
cargo install flip-link
```

## Running

For a debug build
```
DEFMT_LOG=trace cargo run
```
For a release build
```
DEFMT_LOG=trace cargo run --release
```
  
## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

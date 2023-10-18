# Project template for rp2040-hal

This template is intended as a starting point for developing your own firmware based on the rp2040-hal.

It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

`probe-rs` is configured as the default runner, so you can start your program as easy as
```sh
cargo run --release
```

If you aren't using a debugger (or want to use other debugging configurations), check out [alternative runners](#alternative-runners) for other options

<!-- TABLE OF CONTENTS -->
<details open="open">
  
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#markdown-header-requirements">Requirements</a></li>
    <li><a href="#installation-of-development-dependencies">Installation of development dependencies</a></li>
    <li><a href="#running">Running</a></li>
    <li><a href="#alternative-runners">Alternative runners</a></li>
    <li><a href="#notes-on-using-rp2040_boot2">Notes on using rp2040_boot2</a></li>
    <li><a href="#feature-flags">Feature flags</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#code-of-conduct">Code of conduct</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>

<!-- Requirements -->
<details open="open">
  <summary><h2 style="display: inline-block" id="requirements">Requirements</h2></summary>
  
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- (by default) A [`probe-rs` installation](https://probe.rs/docs/getting-started/installation/)

- A [`probe-rs` compatible](https://probe.rs/docs/getting-started/probe-setup/) probe

  You can use a second
  [Pico as a CMSIS-DAP debug probe](debug_probes.md#raspberry-pi-pico). Details
  on other supported debug probes can be found in
  [debug_probes.md](debug_probes.md)

</details>

<!-- Installation of development dependencies -->
<details open="open">
  <summary><h2 style="display: inline-block" id="installation-of-development-dependencies">Installation of development dependencies</h2></summary>

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# Installs the probe-rs tools, including probe-rs run, our recommended default runner
cargo install probe-rs --features=cli --locked
# If you want to use elf2uf2-rs instead, do...
cargo install elf2uf2-rs --locked
```
If you get the error ``binary `cargo-embed` already exists`` during installation of probe-rs, run `cargo uninstall cargo-embed` to uninstall your older version of cargo-embed before trying again.

</details>


<!-- Running -->
<details open="open">
  <summary><h2 style="display: inline-block" id="running">Running</h2></summary>
  
For a debug build
```sh
cargo run
```
For a release build
```sh
cargo run --release
```

If you do not specify a DEFMT_LOG level, it will be set to `debug`.
That means `println!("")`, `info!("")` and `debug!("")` statements will be printed.
If you wish to override this, you can change it in `.cargo/config.toml` 
```toml
[env]
DEFMT_LOG = "off"
```
You can also set this inline (on Linux/MacOS)  
```sh
DEFMT_LOG=trace cargo run
```

or set the _environment variable_ so that it applies to every `cargo run` call that follows:
#### Linux/MacOS/unix
```sh
export DEFMT_LOG=trace
```

Setting the DEFMT_LOG level for the current session  
for bash
```sh
export DEFMT_LOG=trace
```

#### Windows
Windows users can only override DEFMT_LOG through `config.toml`
or by setting the environment variable as a separate step before calling `cargo run`
- cmd
```cmd
set DEFMT_LOG=trace
```
- powershell
```ps1
$Env:DEFMT_LOG = trace
```

```cmd
cargo run
```

</details>
<!-- ALTERNATIVE RUNNERS -->
<details open="open">
  <summary><h2 style="display: inline-block" id="alternative-runners">Alternative runners</h2></summary>

If you don't have a debug probe or if you want to do interactive debugging you can set up an alternative runner for cargo.  

Some of the options for your `runner` are listed below:

* **`cargo embed`**
  This is basically a more configurable version of `probe-rs run`, our default runner.
  See [the `cargo-embed` tool docs page](https://probe.rs/docs/tools/cargo-embed/) for
  more information.
  
  *Step 1* - Install `cargo-embed`. This is part of the [`probe-rs`](https://crates.io/crates/probe-rs) tools:

  ```console
  $ cargo install probe-rs --features=cli --locked
  ```

  *Step 2* - Update settings in [Embed.toml](./Embed.toml)  
  - The defaults are to flash, reset, and start a defmt logging session
  You can find all the settings and their meanings [in the probe-rs repo](https://github.com/probe-rs/probe-rs/blob/c0610e98008cbb34d0dc056fcddff0f2d4f50ad5/probe-rs/src/bin/probe-rs/cmd/cargo_embed/config/default.toml)

  *Step 3* - Use the command `cargo embed`, which will compile the code, flash the device
  and start running the configuration specified in Embed.toml

  ```console
  $ cargo embed --release
  ```

* **probe-rs-debugger**
  *Step 1* - Install Visual Studio Code from https://code.visualstudio.com/

  *Step 2* - Install `probe-rs`
  ```console
  $ cargo install probe-rs --features=cli --locked
  ```

  *Step 3* - Open this project in VSCode

  *Step 4* - Install `debugger for probe-rs` via the VSCode extensions menu (View > Extensions)

  *Step 5* - Launch a debug session by choosing `Run`>`Start Debugging` (or press F5)

* **Loading a UF2 over USB**  
  *Step 1* - Install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs):

  ```console
  $ cargo install elf2uf2-rs --locked
  ```

  *Step 2* - Modify `.cargo/config` to change the default runner

  ```toml
  [target.`cfg(all(target-arch = "arm", target_os = "none"))`]
  runner = "elf2uf2-rs -d"
  ```

  The all-Arm wildcard `'cfg(all(target_arch = "arm", target_os = "none"))'` is used
  by default in the template files, but may also be replaced by
  `thumbv6m-none-eabi`.

  *Step 3* - Boot your RP2040 into "USB Bootloader mode", typically by rebooting
  whilst holding some kind of "Boot Select" button. On Linux, you will also need
  to 'mount' the device, like you would a USB Thumb Drive.

  *Step 4* - Use `cargo run`, which will compile the code and start the
  specified 'runner'. As the 'runner' is the `elf2uf2-rs` tool, it will build a UF2
  file and copy it to your RP2040.

  ```console
  $ cargo run --release
  ```

* **Loading with picotool**  
  As ELF files produced by compiling Rust code are completely compatible with ELF
  files produced by compiling C or C++ code, you can also use the Raspberry Pi
  tool [picotool](https://github.com/raspberrypi/picotool). The only thing to be
  aware of is that picotool expects your ELF files to have a `.elf` extension, and
  by default Rust does not give the ELF files any extension. You can fix this by
  simply renaming the file.

  This means you can't easily use it as a cargo runner - yet.

  Also of note is that the special
  [pico-sdk](https://github.com/raspberrypi/pico-sdk) macros which hide
  information in the ELF file in a way that `picotool info` can read it out, are
  not supported in Rust. An alternative is TBC.

</details>
<!-- Notes on using rp2040_hal and rp2040_boot2 -->
<details open="open">
  <summary><h2 style="display: inline-block" id="notes-on-using-rp2040_boot2">Notes on using rp2040_boot2</h2></summary>

  The second-stage boot loader must be written to the .boot2 section. That
  is usually handled by the board support package (e.g.`rp-pico`). If you don't use
  one, you should initialize the boot loader manually. This can be done by adding the
  following to the beginning of main.rs:
  ```rust
  use rp2040_boot2;
  #[link_section = ".boot2"]
  #[used]
  pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
  ```

</details>

<!-- Feature flags -->
<details open="open">
  <summary><h2 style="display: inline-block" id="feature-flags">Feature flags</h2></summary>

  There are several [feature flags in rp2040-hal](https://docs.rs/rp2040-hal/latest/rp2040_hal/#crate-features).
  If you want to enable some of them, uncomment the `rp2040-hal` dependency in `Cargo.toml` and add the
  desired feature flags there. For example, to enable ROM functions for f64 math using the feature `rom-v2-intrinsics`:
  ```
  rp2040-hal = { version="0.9", features=["rt", "critical-section-impl", "rom-v2-intrinsics"] }
  ```
</details>

<!-- ROADMAP -->

## Roadmap

NOTE These packages are under active development. As such, it is likely to
remain volatile until a 1.0.0 release.

See the [open issues](https://github.com/rp-rs/rp2040-project-template/issues) for a list of
proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

The steps are:

1. Fork the Project by clicking the 'Fork' button at the top of the page.
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Make some changes to the code or documentation.
4. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the Feature Branch (`git push origin feature/AmazingFeature`)
6. Create a [New Pull Request](https://github.com/rp-rs/rp-hal/pulls)
7. An admin will review the Pull Request and discuss any changes that may be required.
8. Once everyone is happy, the Pull Request can be merged by an admin, and your work is part of our project!

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], and the maintainer of this crate, the [rp-rs team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[rp-rs team]: https://github.com/orgs/rp-rs/teams/rp-rs

## License

The contents of this repository are dual-licensed under the _MIT OR Apache
2.0_ License. That means you can chose either the MIT licence or the
Apache-2.0 licence when you re-use this code. See `MIT` or `APACHE2.0` for more
information on each specific licence.

Any submissions to this project (e.g. as Pull Requests) must be made available
under these terms.

## Contact

Raise an issue: [https://github.com/rp-rs/rp2040-project-template/issues](https://github.com/rp-rs/rp2040-project-template/issues)
Chat to us on Matrix: [#rp-rs:matrix.org](https://matrix.to/#/#rp-rs:matrix.org)

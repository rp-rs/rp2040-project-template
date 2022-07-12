# Project template for rp2040-hal

This template is intended as a starting point for developing your own firmware based on the rp2040-hal.

It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

`probe-run` is configured as the default runner, so you can start your program as easy as
```sh
cargo run --release
```

If you aren't using a debugger (or want to use cargo-embed/probe-rs-debugger), check out [alternative runners](#alternative-runners) for other options

<!-- TABLE OF CONTENTS -->
<details open="open">
  
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#markdown-header-requirements">Requirements</a></li>
    <li><a href="#installation-of-development-dependencies">Installation of development dependencies</a></li>
    <li><a href="#running">Running</a></li>
    <li><a href="#alternative-runners">Alternative runners</a></li>
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

- probe-run. Upstream support for RP2040 was added with version 0.3.1.

- A CMSIS-DAP probe. (J-Link and other probes will not work with probe-run)

  You can use a second Pico as a CMSIS-DAP debug probe by installing the following firmware on it:
  https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2

  More details on supported debug probes can be found in [debug_probes.md](debug_probes.md)

</details>

<!-- Installation of development dependencies -->
<details open="open">
  <summary><h2 style="display: inline-block" id="installation-of-development-dependencies">Installation of development dependencies</h2></summary>

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# This is our suggested default 'runner'
cargo install probe-run
# If you want to use elf2uf2-rs instead of probe-run, instead do...
cargo install elf2uf2-rs --locked
```

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

* **cargo embed**  
  *Step 1* - Install [`cargo embed`](https://github.com/probe-rs/cargo-embed):

  ```console
  $ cargo install cargo-embed
  ```

  *Step 2* - Make sure your .cargo/config contains the following

  ```toml
  [target.thumbv6m-none-eabi]
  runner = "cargo embed"
  ```

  *Step 3* - Update settings in [Embed.toml](./Embed.toml)  
  - The defaults are to flash, reset, and start a defmt logging session
  You can find all the settings and their meanings [in the cargo-embed repo](https://github.com/probe-rs/cargo-embed/blob/master/src/config/default.toml)

  *Step 4* - Use `cargo run`, which will compile the code and start the
  specified 'runner'. As the 'runner' is cargo embed, it will flash the device
  and start running immediately

  ```console
  $ cargo run --release
  ```

* **probe-rs-debugger**

  *Step 1* - Download [`probe-rs-debugger VSCode plugin 0.4.0`](https://github.com/probe-rs/vscode/releases/download/v0.4.0/probe-rs-debugger-0.4.0.vsix)

  *Step 2* - Install `probe-rs-debugger VSCode plugin`
  ```console
  $ code --install-extension probe-rs-debugger-0.4.0.vsix
  ```

  *Step 3* - Install `probe-rs-debugger`
  ```console
  $ cargo install probe-rs-debugger
  ```

  *Step 4* - Open this project in VSCode

  *Step 5* - Launch a debug session by choosing `Run`>`Start Debugging` (or press F5)

* **Loading a UF2 over USB**  
  *Step 1* - Install [`elf2uf2-rs`](https://github.com/JoNil/elf2uf2-rs):

  ```console
  $ cargo install elf2uf2-rs --locked
  ```

  *Step 2* - Make sure your .cargo/config contains the following

  ```toml
  [target.thumbv6m-none-eabi]
  runner = "elf2uf2-rs -d"
  ```

  The `thumbv6m-none-eabi` target may be replaced by the all-Arm wildcard
  `'cfg(all(target_arch = "arm", target_os = "none"))'`.

  *Step 3* - Boot your RP2040 into "USB Bootloader mode", typically by rebooting
  whilst holding some kind of "Boot Select" button. On Linux, you will also need
  to 'mount' the device, like you would a USB Thumb Drive.

  *Step 4* - Use `cargo run`, which will compile the code and start the
  specified 'runner'. As the 'runner' is the elf2uf2-rs tool, it will build a UF2
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

# svlint

SystemVerilog linter compliant with IEEE1800-2017.
Written in Rust, based on [sv-parser](https://github.com/dalance/sv-parser).
Svlint is also integrated with most text editors via
[svls](https://github.com/dalance/svls).

[![Actions Status](https://github.com/dalance/svlint/workflows/Regression/badge.svg)](https://github.com/dalance/svlint/actions)
[![codecov](https://codecov.io/gh/dalance/svlint/branch/master/graph/badge.svg)](https://codecov.io/gh/dalance/svlint)

[![Crates.io](https://img.shields.io/crates/v/svlint.svg)](https://crates.io/crates/svlint)
[![svlint](https://snapcraft.io/svlint/badge.svg)](https://snapcraft.io/svlint)

![svlint](https://user-images.githubusercontent.com/4331004/67759664-377b5480-fa83-11e9-895f-7deef6dde516.png)


## Installation

svlint can be installed in several ways:
- Download a [release](https://github.com/dalance/svlint/releases/latest),
  extract, and add the `bin/` directory to your `$PATH`.
  A PDF copy of the MANUAL is included in the `doc/` directory.
- If you have a [Rust toolchain](https://www.rust-lang.org/tools/install), then
  you can install the binary with [cargo](https://crates.io/crates/svlint), via
  `cargo install svlint`.
  This will copy the `svlint` binary (and the dev-only `mdgen` binary) to
  ([usually](https://doc.rust-lang.org/cargo/commands/cargo-install.html#description))
  `~/.cargo/bin`, but not the wrapper scripts (e.g. `svlint-parseonly`) or
  pre-written configurations (e.g. `parseonly.toml`) from `rulesets/`.
- [snapcraft](https://snapcraft.io/svlint), via
  `sudo snap install svlint`.


## Usage

To see information about the command line interface use `svlint --help`,
and to see which version you're running use `svlint --version`.

Further information on how svlint works, how to configure it, and other usage
information is in the [manual](./MANUAL.md).

# firmware
This is my keyboard firmware, intended to run on an RP2040.

## Dependencies
To get a working development environment, simply run `nix develop`. This should give you a development shell with all the dependencies.

If you do not wish to use nix, simply install stable rust, being sure to also install  `rust-src` and the `thumbv6m-none-eabi` target.
## Building
```bash
$ cargo build
$ cargo run # to run on a mounted RP2040 with UF2
```
## Configuration
TODO

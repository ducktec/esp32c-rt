> # :warning: Abandoned!
>
>**This implementation (in combination with [`esp32c3-hal`](https://github.com/ducktec/esp32c3-hal)) was abandoned in favour of the implementation in [esp-rs/esp-hal](https://github.com/esp-rs/esp-hal) and it's use of the default `riscv-rt` crate. The (already more mature) implementation in `esp-hal` supports multiple ESP32 variants, the `ESP32-C3` variant being one of them. It is strongly recommended to use `esp-hal` instead!**

# `esp32c-rt`

> Minimal runtime / startup for RISC-V-based CPU's of the ESP32-C SoC series.

[![Build Status](https://github.com/ducktec/esp32c-rt/actions/workflows/ci.yaml/badge.svg)](https://github.com/ducktec/esp32c-rt/actions/workflows/ci.yaml)
[![crates.io](https://img.shields.io/crates/v/esp32c-rt)](https://crates.io/crates/esp32c-rt)
[![API](https://docs.rs/esp32c-rt/badge.svg)](https://docs.rs/esp32c-rt)
[![License](https://img.shields.io/crates/l/esp32c-rt)](https://github.com/ducktec/esp32c-rt/blob/trunk/LICENSE.md)

This project is a fork of the [`riscv-rt`] crate and provides modifications specific
to the ESP32-C series on top of this crate.

At this point in time, only the [ESP32-C3] has been released. However,
in the future a ESP32-C6 version is planned and it is expected that this
SoC will also be compatible with this crate.

## Modifications

Compared to the `riscv-rt` crate, the following modifications have been implemented:
- Only prepare the binary blobs containing the reset and startup routine for the 
  `rv32imc` architecture. This is the ESP32-C SoC series architecture.
- Exclude `mie` and `mip` machine interrupt registers from the the startup reset
  routine. These registers are not implemented by the SoCs and would result in
  a `invalid instruction` exception if not removed.
- Add a feature `directboot` (by default enabled) that alters the link script so that
  magic numbers are included that boot the EPS32-C SoC in direct-boot mode. This
  simplifies the boot and setup effort significantly, but breaks compatibility with
  numerous [ESP-IDF] features (secure boot etc.). More details on the direct-boot mode
  are available in the [esp32c3-direct-boot-example] repository.
  **This mode is only supported for SoC >= rev. 3**
- Add a feature `esp32c3` (by default enabled at this point in time) that includes
  the memory layout of the ESP32-C3 for the linker so that this file does not have
  to be supplemented by downstream projects.


## ESP32-C3 Memory Layout

The following memory layout is assumed when the feature `esp32c3` is enabled:
```
MEMORY
{
    irom (x): ORIGIN = 0x42000000, LENGTH = 0x400000
    drom (r): ORIGIN = 0x3C000000, LENGTH = 0x400000
    ram (rw): ORIGIN = 0x3FC80000, LENGTH = 0x50000
    rtc_ram (rx): ORIGIN = 0x50000000, LENGTH = 0x2000
}
```

## Documentation

The rust documentation can be found [here](https://docs.rs/crate/esp32c-rt)

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.42.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

See [`LICENSE.md`](LICENSE.md)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
in the repository by you, shall also be licensed under the ISC license as described in
[`LICENSE.md`](LICENSE.md), without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises to intervene to uphold
that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
[`riscv-rt`]: https://github.com/rust-embedded/riscv-rt
[ESP32-C3]: https://www.espressif.com/en/products/socs/esp32-c3
[ESP-IDF]: https://github.com/espressif/esp-idf
[esp32c3-direct-boot-example]: https://github.com/espressif/esp32c3-direct-boot-example
<!-- [![crates.io](https://img.shields.io/crates/d/riscv-rt.svg)](https://crates.io/crates/riscv-rt)
[![crates.io](https://img.shields.io/crates/v/riscv-rt.svg)](https://crates.io/crates/riscv-rt)
[![Build Status](https://travis-ci.org/rust-embedded/riscv-rt.svg?branch=master)](https://travis-ci.org/rust-embedded/riscv-rt) -->

# `esp32c-rt`

> Minimal runtime / startup for RISC-V-based CPU's of the ESP32-C SoC series.

This project is based on the [`riscv-rt`] crate and provides modifications specific
to the ESP32-C series on top of this crate.

At this point in time, only the [ESP32-C3] has been released. However,
in the future a ESP32-C6 version is planned and it is expected that this
SoC will also be compatible with this crate.

## Modifications

In relation to the `riscv-rt` crate, the following modifications have been implemented:
- Only prepare the binary blobs containing the reset and startup routine for the 
  `rv32imc` architecture. This is the ESP32-C SoC series architecture.

## Documentation

The rust documentation can be found [here](https://docs.rs/crate/esp32c-rt)

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.42.0 and up. It *might*
compile with older versions but that may change in any new patch release.

## License

The upstream `riscv-rt` crate is licensed under the following terms:
```
Copyright 2018 [RISC-V team][team]

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```

The modifications in this crate are licensed under the same license:
```
Copyright 2021 Robert Wiewel

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```


## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises to intervene to uphold
that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
[`riscv-rt`]: https://github.com/rust-embedded/riscv
[ESP32-C3]: https://www.espressif.com/en/products/socs/esp32-c3
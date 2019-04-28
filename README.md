# micromath.rs

[![Crate][crate-img]][crate-link]
[![Docs][docs-img]][docs-link]
[![Build Status][build-image]][build-link]
[![LGPL 3.0 licensed][license-image]][license-link]
[![Gitter Chat][gitter-image]][gitter-link]

Embedded Rust math library featuring fast, safe floating point approximations
for common arithmetic operations, 2D and 3D vector types, and statistical
analysis.

[Documentation][docs-link]

## Features

- `f32` extension:
  - Fast approximations:
    - [ ] `asin`
    - [ ] `acos`
    - [ ] `atan`
    - [ ] `atan2`
    - [ ] `cos`
    - [ ] `hypot`
    - [ ] `invsqrt`
    - [ ] `ln`
    - [ ] `log`
    - [ ] `log2`
    - [ ] `log10`
    - [ ] `powf`
    - [ ] `sin`
    - [ ] `sqrt`
    - [ ] `tan`
  - `std` polyfills:
    - [ ] `abs`
    - [ ] `ceil`
    - [ ] `floor`
    - [ ] `round`
    - [ ] `trunc`
- Algebraic vector types:
  - 2D:
    - [ ] `I8x2`
    - [ ] `I16x2`
    - [ ] `I32x2`
    - [ ] `U8x2`
    - [ ] `U16x2`
    - [ ] `U32x2`
    - [ ] `F32x2`
  - 3D:
    - [ ] `I8x3`
    - [ ] `I16x3`
    - [ ] `I32x3`
    - [ ] `U8x3`
    - [ ] `U16x3`
    - [ ] `U32x3`
    - [ ] `F32x3`
- Statistical analysis:
  - `mean`
  - `variance`
  - `stddev`
- Quaternions
  - [ ] TBD

## Comparisons with other Rust crates

### `libm` crate

The [libm crate] contains a port of MUSL's libm to Rust, providing
high-precision implementations of common arithmetic functions.

`micromath` trades precision for performance, instead using the best-available
approximations of the same functions, implemented using safe conversions
between `f32` and `u32`.

The approximations are generally calculated using a combination of bit
twiddling and magic constants, as opposed to the FPU-heavy approaches used by
`libm`. These approaches are culled from recent academic research papers as
well as older approaches which have been commonly used in games and other
performance critical use cases where approximations are adequate.

### `vek` crate

The [vek crate] provides a rich library for 2D and 3D vector types.
Unfortunately, due to a [number of issues](https://github.com/yoanlcq/vek/issues/20)
including a transitive `std` dependency through the [approx crate],
`vek` does not support `no_std`. According to the crate's author, the potential
fixes are nontrivial (and involve addressing problems such as transcendantal
functions causing overflow panics).

`micromath` has been written from the ground up to support `no_std` use cases.
In fact, it doesn't even have a `std` feature!

## Code of Conduct

We abide by the [Contributor Covenant][cc] and ask that you do as well.

For more information, please see [CODE_OF_CONDUCT.md].

## License

Copyright © 2019 NeoBirth Developers

Dual licensed under your choice of either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[crate-img]: https://img.shields.io/crates/v/micromath.svg
[crate-link]: https://crates.io/crates/micromath
[docs-img]: https://docs.rs/micromath/badge.svg
[docs-link]: https://docs.rs/micromath/
[build-image]: https://secure.travis-ci.org/NeoBirth/micromath.svg?branch=master
[build-link]: https://travis-ci.org/NeoBirth/micromath
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/NeoBirth/micromath/blob/master/LICENSE-APACHE
[gitter-image]: https://badges.gitter.im/NeoBirth/micromath.svg
[gitter-link]: https://gitter.im/NeoBirth/community
[libm crate]: https://github.com/rust-lang-nursery/libm
[vek crate]: https://github.com/yoanlcq/vek
[approx crate]: https://crates.io/crates/approx
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/micromath/blob/master/CODE_OF_CONDUCT.md

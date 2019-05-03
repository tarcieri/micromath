# <img src="https://raw.githubusercontent.com/NeoBirth/micromath/develop/img/micromath.png" width="640" height="320">

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
    - [x] [atan]
    - [x] [atan2]
    - [ ] `cos`
    - [ ] `hypot`
    - [ ] `invsqrt`
    - [ ] `ln`
    - [ ] `log`
    - [ ] `log2`
    - [ ] `log10`
    - [ ] `powf`
    - [ ] `sin`
    - [x] [sqrt]
    - [ ] `tan`
  - `std` polyfills:
    - [x] [abs]
    - [ ] `ceil`
    - [ ] `floor`
    - [ ] `round`
    - [ ] `trunc`
- Algebraic vector types:
  - 2D:
    - [x] [I8x2]
    - [x] [I16x2]
    - [ ] `I32x2`
    - [x] [U8x2]
    - [x] [U16x2]
    - [ ] `U32x2`
    - [x] [F32x2]
  - 3D:
    - [x] [I8x3]
    - [x] [I16x3]
    - [ ] `I32x3`
    - [x] [U8x3]
    - [x] [U16x3]
    - [ ] `U32x3`
    - [x] [F32x3]
- Statistical analysis:
  - [x] [mean]
  - [x] [variance]
  - [x] [stddev]
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

Incorporates some tests from the [libm crate], which is
Copyright © 2018 Jorge Aparicio and also dual licensed under the
Apache 2.0 and MIT licenses. 

[crate-img]: https://img.shields.io/crates/v/micromath.svg
[crate-link]: https://crates.io/crates/micromath
[docs-img]: https://docs.rs/micromath/badge.svg
[docs-link]: https://docs.rs/micromath/
[build-image]: https://travis-ci.com/NeoBirth/micromath.svg?branch=develop
[build-link]: https://travis-ci.com/NeoBirth/micromath
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[license-link]: https://github.com/NeoBirth/micromath/blob/master/LICENSE-APACHE
[gitter-image]: https://badges.gitter.im/NeoBirth/micromath.svg
[gitter-link]: https://gitter.im/NeoBirth/community
[atan]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.atan
[atan2]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.atan2
[sqrt]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.sqrt
[abs]: https://docs.rs/micromath/latest/micromath/trait.F32Ext.html#tymethod.abs
[I8x2]: https://docs.rs/micromath/latest/micromath/vector/struct.I8x2.html
[I16x2]: https://docs.rs/micromath/latest/micromath/vector/struct.I16x2.html
[U8x2]: https://docs.rs/micromath/latest/micromath/vector/struct.U8x2.html
[U16x2]: https://docs.rs/micromath/latest/micromath/vector/struct.U16x2.html
[F32x2]: https://docs.rs/micromath/latest/micromath/vector/struct.F32x2.html
[I8x3]: https://docs.rs/micromath/latest/micromath/vector/struct.I8x3.html
[I16x3]: https://docs.rs/micromath/latest/micromath/vector/struct.I8x3.html
[U8x3]: https://docs.rs/micromath/latest/micromath/vector/struct.U8x3.html
[U16x3]: https://docs.rs/micromath/latest/micromath/vector/struct.U16x3.html
[F32x3]: https://docs.rs/micromath/latest/micromath/vector/struct.F32x3.html
[mean]: https://docs.rs/micromath/latest/micromath/statistics/trait.Mean.html
[variance]: https://docs.rs/micromath/latest/micromath/statistics/trait.Variance.html
[stddev]: https://docs.rs/micromath/latest/micromath/statistics/trait.StdDev.html
[libm crate]: https://github.com/rust-lang-nursery/libm
[vek crate]: https://github.com/yoanlcq/vek
[approx crate]: https://crates.io/crates/approx
[cc]: https://contributor-covenant.org
[CODE_OF_CONDUCT.md]: https://github.com/NeoBirth/micromath/blob/master/CODE_OF_CONDUCT.md

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2.2.0 (2026-11-21)
### Added
- `exp2` and `sin_turns` on `F32` and `F32Ext` for octave-based pitch and gain calculations and oscillators with phase measured in turns ([#128])
- `F32::cosh` for the hyperbolic cosine ([#113])
- `Vector::map`, `Vector::normalized`, and `Vector::magnitude_sq` ([#114], [#118])
- Dot and cross products, `Iterator::sum`, scalar-left multiplication, and scalar division for `Vector2d` and `Vector3d` ([#112], [#115], [#116], [#117])
- Array and tuple conversions for vectors and conversions between `Quaternion` and `[f32; 4]` ([#111])
- Optional `defmt` formatting for `F32`, vectors, and quaternions ([#108])

### Changed
- Raise the minimum supported Rust version from 1.47 to 1.62 ([#108])
- Improve `tan` accuracy near zero and near its poles, and improve range reduction for large angles ([#127])
- Improve `exp` accuracy near overflow and underflow. `log2` now returns exact results for every representable power of two ([#128])

### Fixed
- Correct `ln` for inputs between zero and one, including subnormals. Both signed zeros now produce negative infinity ([#125])
- Handle the full `i32` exponent range in `powi`, including `i32::MIN`, and avoid spurious overflow for finite results ([#126])
- Use `powi` for integer-valued `powf` exponents that fit in `i32`, fixing rounding at the subnormal boundary ([#128])
- Preserve signed zero, large integral values, infinities, and NaNs in `floor` ([#128])
- Avoid integer underflow in `F32::inv` when overflow checks are enabled. This change has a known release-mode regression for negative inputs, where `F32(-2.0).inv()` now returns `+0.0` ([#110])

[#108]: https://github.com/tarcieri/micromath/pull/108
[#110]: https://github.com/tarcieri/micromath/pull/110
[#111]: https://github.com/tarcieri/micromath/pull/111
[#112]: https://github.com/tarcieri/micromath/pull/112
[#113]: https://github.com/tarcieri/micromath/pull/113
[#114]: https://github.com/tarcieri/micromath/pull/114
[#115]: https://github.com/tarcieri/micromath/pull/115
[#116]: https://github.com/tarcieri/micromath/pull/116
[#117]: https://github.com/tarcieri/micromath/pull/117
[#118]: https://github.com/tarcieri/micromath/pull/118
[#125]: https://github.com/tarcieri/micromath/pull/125
[#126]: https://github.com/tarcieri/micromath/pull/126
[#127]: https://github.com/tarcieri/micromath/pull/127
[#128]: https://github.com/tarcieri/micromath/pull/128

## 2.1.0 (2023-10-31)
### Added
- Accessors to quaternion elements ([#89])
- Quaternion -> Euler conversion ([#90])
- Method for normalizing a quaternion ([#91])
- Method to get the quaternion which is the smallest rotation between two vectors ([#94])
- `recip`, `mul_add` and `signum` ([#100])

[#89]: https://github.com/tarcieri/micromath/pull/89
[#90]: https://github.com/tarcieri/micromath/pull/90
[#91]: https://github.com/tarcieri/micromath/pull/91
[#94]: https://github.com/tarcieri/micromath/pull/94
[#100]: https://github.com/tarcieri/micromath/pull/100

## 2.0.0 (2021-05-15)
### Added
- `Vector` subtraction support ([#71])
- `F32` newtype ([#72], [#75])
- `num-traits` support ([#80])
- `Quaternion::dot` and `::inv` ([#81])
- `Vector3d` ops for `Quaternion` ([#82])
- `Quaternion::magnitude`, `::scale`, `::to_array`, and `::IDENTITY` ([#83])
- `Quaternion::axis_angle` ([#84])
- `Quaternion::new` ([#85])

### Changed
- Refactor `Vector` types ([#69])
- MSRV 1.47+ ([#75])
- Make `Quaternion` opaque and module private ([#70], [#85])

### Fixed
- `acos()` behavior for zero/negative ([#79])

[#69]: https://github.com/tarcieri/micromath/pull/69
[#70]: https://github.com/tarcieri/micromath/pull/70
[#71]: https://github.com/tarcieri/micromath/pull/71
[#72]: https://github.com/tarcieri/micromath/pull/72
[#75]: https://github.com/tarcieri/micromath/pull/75
[#79]: https://github.com/tarcieri/micromath/pull/79
[#80]: https://github.com/tarcieri/micromath/pull/80
[#81]: https://github.com/tarcieri/micromath/pull/81
[#82]: https://github.com/tarcieri/micromath/pull/82
[#83]: https://github.com/tarcieri/micromath/pull/83
[#84]: https://github.com/tarcieri/micromath/pull/84
[#85]: https://github.com/tarcieri/micromath/pull/85

## 1.1.1 (2021-03-27)
### Added
- `doc_cfg` ([#64])

[#64]: https://github.com/tarcieri/micromath/pull/64

## 1.1.0 (2020-09-30)
### Added
- `powi` support ([#53])

### Changed
- Bump `generic-array` dependency to v0.14; MSRV 1.36+ ([#54])

[#54]: https://github.com/tarcieri/micromath/pull/54
[#53]: https://github.com/tarcieri/micromath/pull/53

## 1.0.1 (2020-06-12)
### Added
- Support for `powf` with negative bases ([#51])

[#51]: https://github.com/tarcieri/micromath/pull/51

## 1.0.0 (2019-12-02)
- Initial 1.0 release! 🎉 (otherwise unchanged)

## 0.5.1 (2019-11-27)
- Cargo.toml: Add mathematics category ([#45])

[#45]: https://github.com/tarcieri/micromath/pull/45

## 0.5.0 (2019-11-13)
- Remove default cargo features ([#42])
- Add `asin`, `acos`, and `hypot` ([#39])

[#42]: https://github.com/tarcieri/micromath/pull/42
[#39]: https://github.com/tarcieri/micromath/pull/39

## 0.4.1 (2019-10-08)
- Implement `F32Ext::round` ([#37])

[#37]: https://github.com/tarcieri/micromath/pull/37

## 0.4.0 (2019-10-02)
- Add `powf`, `exp`, `log10`, `log2`, `log`, `ln`, `trunc`, `fract`, `copysign` ([#35])

[#35]: https://github.com/tarcieri/micromath/pull/35

## 0.3.1 (2019-05-11)
- Rust 1.31.0 support ([#33])

[#33]: https://github.com/tarcieri/micromath/pull/33

## 0.3.0 (2019-05-04)
- statistics: Add Trim trait for statistical outlier culling iterators ([#29])
- Quaternions ([#28])
- f32ext: fast `inv()` approximation ([#27])
- Improve documentation throughout the library ([#25], [#26])

[#29]: https://github.com/tarcieri/micromath/pull/29
[#28]: https://github.com/tarcieri/micromath/pull/28
[#27]: https://github.com/tarcieri/micromath/pull/27
[#26]: https://github.com/tarcieri/micromath/pull/26
[#25]: https://github.com/tarcieri/micromath/pull/25

## 0.2.2 (2019-05-04)
- Add `i32` and `u32` vectors ([#23])

[#23]: https://github.com/tarcieri/micromath/pull/23

## 0.2.1 (2019-05-03)
- Add `html_logo_url` and square icon ([#20])
- `README.md`: Update links to use 'develop' branch ([#19])

[#20]: https://github.com/tarcieri/micromath/pull/20
[#19]: https://github.com/tarcieri/micromath/pull/19

## 0.2.0 (2019-05-03)
- `tan(x)` ([#17])
- `invsqrt(x)` ([#16])
- `cos(x)` and `sin(x)` ([#15])
- `ceil(x)` and `floor(x)` ([#14])
- Update to `generic-array` v0.13 ([#12])

[#17]: https://github.com/tarcieri/micromath/pull/17
[#16]: https://github.com/tarcieri/micromath/pull/16
[#15]: https://github.com/tarcieri/micromath/pull/15
[#14]: https://github.com/tarcieri/micromath/pull/14
[#12]: https://github.com/tarcieri/micromath/pull/12

## 0.1.0 (2019-05-03)
- Initial release

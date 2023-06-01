# Micromod Benchmark Suite

*- intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) board -*

## Prerequisites

The following hardware is required for the benchmark:
- A [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) development board
- A way to read the Teensy's UART, like a USB-UART-converter

The following software tools have to be installed:
- Python3 (as `python`, or modify `run.py` to use the `python3` binary)
- [`llvm-objcopy`](https://github.com/rust-lang/rust/issues/85658)
  - Install via `rustup component add llvm-tools-preview`
- [`teensy-loader-cli`](https://www.pjrc.com/teensy/loader_cli.html)


## Run

- Connect the Teensy to PC via USB cable.
- Run `cargo run --release`.
- Read the output of the benchmark on the Teensy's UART.

## Results

```none
===== Micromath Benchmark =====
Git Version: deb8988

All values in ns/iter.

            micromath       libm intrinsics
abs              15.0       15.0       10.0
acos            173.3      120.0
asin            103.3      136.7
atan             80.0       78.3
atan_norm        75.0
ceil             43.3       41.7
cos              98.3     1655.0
exp             231.7      115.0
floor            36.7       38.3
fract            20.0
inv              15.0
invsqrt          15.0
ln              158.3      145.0
log2            161.7      155.0
log10           161.7      160.0
round            30.0
sin             103.3     1703.3
sqrt             46.7      470.0       31.7
tan             153.3     2531.3
trunc            21.7       43.3
```

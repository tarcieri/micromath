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
Git Version: 8e2f06e

All values in ns/iter.

            micromath       libm intrinsics
abs              15.0       15.0       10.0
acos            173.3       96.7
asin            103.3       85.0
atan             80.0       68.3
atan_norm        75.0
ceil             43.3       38.3
cos              98.3     1603.3
exp             176.7      101.7
floor            36.7       35.0
fract            20.0
inv              15.0
invsqrt          15.0
ln              156.7      141.7
log2            163.3      153.3
log10           163.3      158.3
round            30.0
sin             103.3     1722.8
sqrt             46.7      475.0       31.7
tan             153.3     2524.6
trunc            21.7       43.3
```

# Micromod Benchmark Suite

*- intended for the [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) board -*

## Prerequisites

The following hardware is required for the benchmark:
- A [Teensy 4.0](https://www.pjrc.com/store/teensy40.html) development board
- A way to read the Teensy's UART, like a USB-UART-converter

The following tools have to be installed:
- Python3 (as `python`, or modify `run.py` to use the `python3` binary)
- [`llvm-objcopy`](https://github.com/rust-lang/rust/issues/85658)
  - Install via `rustup component add llvm-tools-preview`
- [`teensy-loader-cli`](https://www.pjrc.com/teensy/loader_cli.html)


## Run

- Run `cargo run --release`.
- Read the output of the benchmark on the Teensy's UART.

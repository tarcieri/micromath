//! Demonstrates a loopback UART peripheral.
//!
//! It uses the alpha board, with the following pinout:
//!
//! - Pin 14 is TX.
//! - Pin 15 is RX.
//!
//! Baud rate is 115200bps.
//!
//! Every time you send the Teensy a character, it replies with
//! that same character, and it toggles the LED.

#![no_std]
#![no_main]
#![feature(core_intrinsics)]

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;
use bsp::hal;

mod uart_writer;
use uart_writer::UartWriter;

mod run_bench;
use run_bench::run_bench;

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        lpuart2,
        gpt1: mut us_timer,
        ..
    } = board::t40(board::instances());

    // Initialize LED
    let led = board::led(&mut gpio2, pins.p13);
    led.set();

    // Initialize UART
    let mut uart = UartWriter::new(board::lpuart(lpuart2, pins.p14, pins.p15, 115200));
    writeln!(uart);

    // Initialize timer
    // Is a 32-bit timer with us precision.
    // Overflows every 71.58 minutes, which is sufficient for our benchmark.
    assert_eq!(board::PERCLK_FREQUENCY, 1_000_000);
    us_timer.set_clock_source(hal::gpt::ClockSource::PeripheralClock);
    us_timer.set_divider(1);
    us_timer.set_mode(hal::gpt::Mode::FreeRunning);
    us_timer.enable();
    let time_us = move || us_timer.count();

    // Write welcome message
    writeln!(uart, "===== Micromath Benchmark =====");
    writeln!(uart, "Git Version: {}", git_version::git_version!());
    writeln!(uart);

    writeln!(uart, "All values in ns/iter.");
    writeln!(uart);
    write!(uart, "          ");
    write!(uart, " micromath");
    write!(uart, "      libm");
    writeln!(uart);

    // Run benchmarks
    run_bench!(time_us, uart, acos, acosf);
    run_bench!(time_us, uart, asin, asinf);
    run_bench!(time_us, uart, atan, atanf);
    run_bench!(time_us, uart, atan_norm);
    run_bench!(time_us, uart, ceil, ceilf);
    run_bench!(time_us, uart, cos, cosf);
    run_bench!(time_us, uart, exp, expf);
    run_bench!(time_us, uart, floor, floorf);
    run_bench!(time_us, uart, fract);
    run_bench!(time_us, uart, inv);
    run_bench!(time_us, uart, invsqrt);
    run_bench!(time_us, uart, ln, logf);
    run_bench!(time_us, uart, log2, log2f);
    run_bench!(time_us, uart, log10, log10f);
    run_bench!(time_us, uart, round, roundf);
    run_bench!(time_us, uart, sin, sinf);
    run_bench!(time_us, uart, sqrt, sqrtf);
    run_bench!(time_us, uart, tan, tanf);
    run_bench!(time_us, uart, trunc, truncf);

    loop {}
}

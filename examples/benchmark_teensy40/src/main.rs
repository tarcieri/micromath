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

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::board;
use bsp::hal;

mod uart_writer;
use uart_writer::UartWriter;

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

    // Write welcome message
    writeln!(uart, "===== Micromath Benchmark =====");
    writeln!(uart, "Git Version: {}", git_version::git_version!());
    writeln!(uart);

    writeln!(uart, "Time: {}", us_timer.count());
    writeln!(uart, "Time: {}", us_timer.count());

    loop {}
}

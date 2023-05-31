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

mod uart_writer;
use uart_writer::UartWriter;

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources { pins, lpuart2, .. } = board::t40(board::instances());

    // Initialize uart
    let mut uart = UartWriter::new(board::lpuart(lpuart2, pins.p14, pins.p15, 115200));
    writeln!(uart);

    // Write welcome message
    writeln!(uart, "===== Micromath Benchmark =====");
    writeln!(uart, "Git Version: {}", git_version::git_version!());

    loop {}
}

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

use embedded_hal::serial::{Read, Write};

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pins,
        mut gpio2,
        lpuart2,
        ..
    } = board::t40(board::instances());
    let led = board::led(&mut gpio2, pins.p13);

    let mut lpuart2: board::Lpuart2 = board::lpuart(lpuart2, pins.p14, pins.p15, 115200);
    loop {
        led.toggle();
        let byte = nb::block!(lpuart2.read()).unwrap();
        nb::block!(lpuart2.write(byte)).unwrap();
    }
}

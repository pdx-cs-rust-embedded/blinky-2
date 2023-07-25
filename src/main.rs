//! Blink MicroBit 2 LED at 1Hz.

#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    // Set up board and peripherals.
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut d = board.display_pins;
    let mut timer = Timer::new(board.TIMER0);

    // Leave column 1 low so that we can turn row 1 on and
    // off when we want. The other LED pins default to high,
    // so we should be OK here.
    d.col1.set_low().unwrap();

    loop {
        // Turn on LED 1, 1.
        d.row1.set_high().unwrap();
        // Block for 500 ms.
        timer.delay_ms(500u16);
        // Turn off LED 1, 1.
        d.row1.set_low().unwrap();
        // Block for 500 ms.
        timer.delay_ms(500u16);
    }
}

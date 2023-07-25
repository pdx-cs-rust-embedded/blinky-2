#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{board::Board, hal::{prelude::*, Timer}};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut d = board.display_pins;
    let mut timer = Timer::new(board.TIMER0);
    
    d.col1.set_low().unwrap();

    loop {
        d.row1.set_high().unwrap();
        timer.delay_ms(500u16);
        d.row1.set_low().unwrap();
        timer.delay_ms(500u16);
    }
}

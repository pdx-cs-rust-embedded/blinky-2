#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{board::Board, hal::prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut col = board.display_pins.col1;
    let mut row = board.display_pins.row1;
    
    col.set_low().unwrap();
    row.set_high().unwrap();

    loop {
        continue;
    }
}

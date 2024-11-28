#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    board.display_pins.col2.set_low().unwrap();
    board.display_pins.row2.set_high().unwrap();

    loop {}
}

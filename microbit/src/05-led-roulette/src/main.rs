#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::Display;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = microbit::display::blocking::Display::new(board.display_pins);
    let mut led_state = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    loop {
        for row_idx in 0..led_state.len() {
            for col_idx in 0..led_state[row_idx].len() {
                let value = &mut led_state[row_idx][col_idx];
                let previous_value = if col_idx > 0 {
                    Some(&mut led_state[row_idx][col_idx - 1])
                } else {
                    None
                };
                match row_idx {
                    0 => {
                        if *previous_value == 1 {
                            *previous_value = 0;
                        }
                        if *value == 1 {
                            *value = 0;
                        } else {
                            *value = 1;
                        }
                        rprintln!("Set led_state[{}][{}] to {}", row_idx, col_idx, *value);
                        timer.delay_ms(300_u16);
                    }
                    _ => {
                        rprintln!("Value at led_state[{}][{}] is {}", row_idx, col_idx, *value);
                        timer.delay_ms(300_u16);
                    }
                }
            }
        }

        display.clear();
        timer.delay_ms(1000_u16);
    }
}

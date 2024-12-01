#![deny(unsafe_code)]
#![no_main]
#![no_std]
mod render;
use crate::{render, LedState};
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
    let mut led_state = [[LedState {
        current: 0,
        previous: 0,
    }; 5]; 5];
    loop {
        for row_idx in 0..led_state.len() {
            for col_idx in 0..led_state[row_idx].len() {
                let led = &mut led_state[row_idx][col_idx];
                match row_idx {
                    0 => {
                        led.toggle();

                        rprintln!(
                            "Row: {}, Col: {}, Current: {}, Previous: {}",
                            row_idx,
                            col_idx,
                            led.current,
                            led.previous
                        );
                        render(&led_state);
                        timer.delay_ms(300_u16);
                    }
                    _ => {
                        rprintln!(
                            "Row: {}, Col: {}, Current: {}, Previous: {}",
                            row_idx,
                            col_idx,
                            led.current,
                            led.previous
                        );
                        timer.delay_ms(300_u16);
                    }
                }
            }
        }
    }
}

use microbit::{board::Board, display::blocking::Display, hal::Timer};

#[derive(Copy, Clone)]
pub struct LedState {
    pub current: u8,
    pub previous: u8,
}
impl LedState {
    pub fn toggle(&mut self) {
        self.previous = self.current;
        self.current = if self.current == 0 { 1 } else { 0 };
    }
}

pub fn render(led_state: &[[LedState; 5]; 5]) {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut screen = [[0; 5]; 5];
    for row_idx in 0..led_state.len() {
        for col_idx in 0..led_state[row_idx].len() {
            screen[row_idx][col_idx] = led_state[row_idx][col_idx].current;
        }
    }
    display.show(&mut timer, screen, 30);
}

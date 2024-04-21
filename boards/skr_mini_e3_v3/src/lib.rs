#![no_std]
#![no_main]

use hf_boards::Board;

pub struct BoardSkrMiniE3V3 {}

#[inline]
pub fn new() -> BoardSkrMiniE3V3 {
    BoardSkrMiniE3V3 {}
}

impl Board for BoardSkrMiniE3V3 {
    fn init(&self) -> embassy_stm32::Peripherals {
        let config = embassy_stm32::Config::default();
        embassy_stm32::init(config)
    }
}

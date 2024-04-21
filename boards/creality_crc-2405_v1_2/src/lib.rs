#![no_std]
#![no_main]

use hf_boards::Board;

pub struct BoardCrealityCrc2405v1_2 {}

#[inline]
pub fn new() -> BoardCrealityCrc2405v1_2 {
    BoardCrealityCrc2405v1_2 {}
}

impl Board for BoardCrealityCrc2405v1_2 {
    fn init(&self) -> embassy_stm32::Peripherals {
        let config = embassy_stm32::Config::default();
        embassy_stm32::init(config)
    }
}

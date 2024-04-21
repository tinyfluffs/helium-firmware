#![no_std]
#![no_main]

pub trait Board {
    fn init(&self) -> embassy_stm32::Peripherals;
}

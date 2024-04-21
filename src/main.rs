#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::{Input, Pull};
use {defmt_rtt as _, panic_probe as _};
use hf_boards::Board;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    #[cfg(feature = "skr_mini_e3_v3")]
    let board = hf_skr_mini_e3_v3::new();

    let p = board.init();

    loop {
    }
}

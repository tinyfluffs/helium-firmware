#![no_std]
#![no_main]

use defmt::*;
use embassy_stm32::gpio::{Input, Pull};
use {defmt_rtt as _, panic_probe as _};
use hf_boards::Board;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) -> ! {
    #[cfg(feature = "btt-skr_mini_e3_v3")]
    let board = hf_btt_skr_mini_e3_v3::new();

    #[cfg(feature = "creality-crc-2405-v1-2")]
    let board = hf_creality_crc_2405_v1_2::new();

    let p = board.init();

    loop {
    }
}

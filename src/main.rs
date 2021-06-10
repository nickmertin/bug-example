#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::{
    dbg,
    debug::{exit, EXIT_SUCCESS},
};
use panic_semihosting as _;
use stm32g0xx_hal as _;

#[entry]
fn main() -> ! {
    dbg!(1i128);
    exit(EXIT_SUCCESS);
    loop {}
}

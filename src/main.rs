#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_semihosting as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let mut led = gpioe
        .pe13 // LD10 in f3discovery board
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    loop {
        led.toggle().unwrap();
        defmt::info!("led!");
        asm::delay(8_000_000);
    }
}

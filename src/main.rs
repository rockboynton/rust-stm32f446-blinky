#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

fn led_button_control() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA.split();

    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = peripherals.GPIOC.split();
    let button = gpioc.pc13;

    loop {
        if button.is_high() {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}

#[rt::entry]
fn main() -> ! {
    led_button_control();
}

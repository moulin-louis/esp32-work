#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::prelude::*;
use log::info;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO7, Level::Low);
    info!("let init");
    led.set_high();
    info!("led set high");

    let delay = Delay::new();
    loop {
        info!("toggle");
        led.toggle();
        info!("delay 500 millis");
        delay.delay(500.millis());
        info!("loop iter done");
    }
}

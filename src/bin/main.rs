#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::Input;
use esp_hal::gpio::{Level, Output};
use esp_hal::i2c::master::*;
use esp_hal::prelude::*;
use esp_hal::time;
use lcd_lcm1602_i2c::sync_lcd::Lcd;
use log::info;

const ADDRESS_LCD: u8 = 0x27;

fn floor(x: f64) -> f64 {
    let int_part = x as i64;
    let float_part = int_part as f64;

    // If negative and has decimal part, round down
    if x < 0.0 && float_part != x {
        float_part - 1.0
    } else {
        float_part
    }
}

fn distance(input_echo: &mut Input, output_trig: &mut Output, delay: &Delay) -> f64 {
    // 10 μS of trigger signal  to ask for measurement
    output_trig.set_high();
    delay.delay(10.micros());
    output_trig.set_low();

    while input_echo.is_low() {}
    let before = time::now();
    while input_echo.is_high() {}
    let after = time::now();

    let diff = after - before;
    floor(17.0f64 / 100.0f64 * diff.to_micros() as f64)
}

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger(log::LevelFilter::Info);
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut input_echo = Input::new(peripherals.GPIO22, esp_hal::gpio::Pull::Up);

    let mut output_trig = Output::new(peripherals.GPIO23, Level::Low);

    let mut i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_sda(peripherals.GPIO14)
        .with_scl(peripherals.GPIO27);

    let mut delay = Delay::new();
    let mut lcd = Lcd::new(&mut i2c, &mut delay)
        .with_address(ADDRESS_LCD)
        .with_cursor_on(false)
        .with_rows(2)
        .init()
        .unwrap();

    let delay = Delay::new();
    loop {
        info!("calling distance...");
        let distance = distance(&mut input_echo, &mut output_trig, &delay);
        lcd.clear().unwrap();
        lcd.write_str("distance = ").unwrap();
        info!("distance = {}", distance);
        lcd.write_str(" cm").unwrap();
        delay.delay(1000.millis());
    }
}

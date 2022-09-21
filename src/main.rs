extern crate core;
use std::{env, thread};
use std::error::Error;
use std::time::Duration;
use log::info;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::uart::{Parity, Uart};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
    info!("Started!");

    let bus = Bus::Spi0;
    let mode = Mode::Mode0;

    // Which device (pin) should listen to the SPI bus.
    // We will be using SS0 pins. i.e. physical pin 21, et al.
    let slave = SlaveSelect::Ss0;

    let clock_speed = 3 * 1000 * 1000;
    let buffer = Vec::new();

    let mut spi:Spi = spi: Spi::new(bus, slave, clock_speed, mode).unwrap();

    spi.write(&buffer).expect("weird error occurred!");

    Ok(())
}
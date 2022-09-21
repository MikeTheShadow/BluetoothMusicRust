extern crate core;

mod lib;

use std::{env, thread};
use std::error::Error;
use std::time::Duration;
use log::info;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::uart::{Parity, Uart};
use crate::lib::LedPanel;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
    info!("Started!");

    let mut buffer = Vec::new();

    let mut panel:  LedPanel =  LedPanel::new(256);

    buffer.append(&mut vec![255,0,0]);

    panel.spi.write(&buffer).expect("panic message");

    Ok(())
}
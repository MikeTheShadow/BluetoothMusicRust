mod lib;

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
use bluetooth_music_rust::LedPanel;
use crate::lib::ColorRGB;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
    info!("Started!");

    let mut panel: LedPanel = LedPanel::new(8 * 32);

    panel.clear_all_leds();

    let leds : [ColorRGB; 1] = [ColorRGB::new(5, 5, 5)];

    panel.set_leds(&leds);

    Ok(())
}
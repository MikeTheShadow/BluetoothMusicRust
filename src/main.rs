extern crate core;

use std::{env, thread};
use std::time::Duration;
use log::info;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Polarity, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::uart::{Parity, Uart};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
    info!("Started!");
    // Enable PWM channel 0 (BCM GPIO 18, physical pin 12) at 2 Hz with a 25% duty cycle.
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true).unwrap();

    // Sleep for 2 seconds while the LED blinks.
    thread::sleep(Duration::from_secs(2));

    // Reconfigure the PWM channel for an 8 Hz frequency, 50% duty cycle.
    pwm.set_frequency(8.0, 0.5)?;

    thread::sleep(Duration::from_secs(3));

}
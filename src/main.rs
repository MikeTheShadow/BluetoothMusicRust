extern crate core;

use std::env;
use log::info;
use rpi_embedded::pwm::{Channel, Pwm};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env::set_var("RUST_LOG", "info");
    pretty_env_logger::init_timed();
    info!("Started!");

    let pi_peripheral : Pwm = Pwm::new(Channel::Pwm0).unwrap()?;
}
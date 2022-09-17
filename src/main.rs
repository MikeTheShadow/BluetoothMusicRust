extern crate core;

use std::fs::read;
use std::future::Future;
use std::pin::Pin;
use std::thread::sleep;
use std::time::Duration;
use bluer::AdapterEvent;
use bluer::adv::Advertisement;
use bluer::agent::{Agent, DisplayPinCode, DisplayPinCodeFn, ReqResult, RequestPinCode, RequestPinCodeFn};
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {

    let request_pin_code: RequestPinCodeFn = Box::new(|request_pin_code| Box::pin(async {
        println!("Hello from request pin code!");
        return Ok(String::from("hello world"));
    }));

    let display_pin_code: DisplayPinCodeFn = Box::new(|display_pin_code| Box::pin(async {
        println!("Hello from display pin code!");
        // println!("Data \nAdapter: {}\nDevice: {}\nCode: {}",&display_pin_code.adapter,&display_pin_code.device,&display_pin_code.pincode);
        return Ok(());
    }));

    let agent:Agent = Agent {
        request_default: true,
        request_pin_code: Some(request_pin_code),
        display_pin_code: Some(display_pin_code),
        request_passkey: None,
        display_passkey: None,
        request_confirmation: None,
        request_authorization: None,
        authorize_service: None,
        _non_exhaustive: ()
    };

    let session = bluer::Session::new().await?;
    session.register_agent(agent);
    // let adapter_names = session.adapter_names().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!("Advertising on Bluetooth adapter {} with address {}", adapter.name(), adapter.address().await?);
    let le_advertisement = Advertisement {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: vec!["123e4567-e89b-12d3-a456-426614174000".parse().unwrap()].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("le_advertise".to_string()),
        ..Default::default()
    };
    println!("{:?}", &le_advertisement);
    let handle = adapter.advertise(le_advertisement).await?;

    println!("Press enter to quit");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing advertisement");
    drop(handle);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
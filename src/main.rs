extern crate core;

use std::fs::read;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use bluer::AdapterEvent;
use bluer::adv::Advertisement;
use bluer::agent::{Agent, DisplayPinCode, DisplayPinCodeFn, ReqResult, RequestPinCode, RequestPinCodeFn};
use tokio::io::{AsyncBufReadExt, BufReader};

async fn query_adapter(adapter: &bluer::Adapter) -> bluer::Result<()> {
    println!("    Address:                    {}", adapter.address().await?);
    println!("    Friendly name:              {}", adapter.alias().await?);
    println!("    Modalias:                   {:?}", adapter.modalias().await?);
    println!("    Powered:                    {:?}", adapter.is_powered().await?);
    println!("    Discoverabe:                {:?}", adapter.is_discoverable().await?);
    println!("    Pairable:                   {:?}", adapter.is_pairable().await?);
    println!("    UUIDs:                      {:?}", adapter.uuids().await?);
    println!();
    println!("    Active adv. instances:      {}", adapter.active_advertising_instances().await?);
    println!("    Supp.  adv. instances:      {}", adapter.supported_advertising_instances().await?);
    println!("    Supp.  adv. includes:       {:?}", adapter.supported_advertising_system_includes().await?);
    println!("    Adv. capabilities:           {:?}", adapter.supported_advertising_capabilities().await?);
    println!("    Adv. features:              {:?}", adapter.supported_advertising_features().await?);

    Ok(())
}

async fn query_all_adapter_properties(adapter: &bluer::Adapter) -> bluer::Result<()> {
    let props = adapter.all_properties().await?;
    for prop in props {
        println!("    {:?}", &prop);
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {

    // pub type RequestPinCodeFn =
    //     Box<dyn (Fn(RequestPinCode) -> Pin<Box<dyn Future<Output = ReqResult<String>> + Send>>) + Send + Sync>;

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
    let adapter_names = session.adapter_names().await?;
    for adapter_name in adapter_names {
        println!("Bluetooth adapter {}:", &adapter_name);
        let adapter = session.adapter(&adapter_name)?;
        let le_advertisement = Advertisement {
            advertisement_type: bluer::adv::Type::Peripheral,
            service_uuids: vec![
                "00001108-0000-1000-8000-00805f9b34fb".parse().unwrap()
                ,"0000110d-0000-1000-8000-00805f9b34fb".parse().unwrap()
            ].into_iter().collect(),
            discoverable: Some(true),
            local_name: Some("le_advertise".to_string()),
            ..Default::default()
        };
        let handle = adapter.advertise(le_advertisement).await?;
        query_adapter(&adapter).await?;
        println!();
    }
    println!("Press enter to quit");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;
    Ok(())
}
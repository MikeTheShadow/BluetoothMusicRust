use std::env;
use std::io::{BufRead, BufReader};
use std::time::Duration;
use bluer::AdapterEvent;
use bluer::adv::Advertisement;

async fn query_adapter(adapter: &bluer::Adapter) -> bluer::Result<()> {
    println!("    Address:                    {}", adapter.address().await?);
    println!("    Address type:               {}", adapter.address_type()?);
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
    println!("    Adv. capabilites:           {:?}", adapter.supported_advertising_capabilities().await?);
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
    let all_properties = env::args().any(|arg| arg == "--all-properties");

    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    for adapter_name in adapter_names {
        println!("Bluetooth adapter {}:", &adapter_name);
        let adapter = session.adapter(&adapter_name)?;
        let le_advertisement = Advertisement {
            advertisement_type: bluer::adv::Type::Peripheral,
            service_uuids: vec!["123e4567-e89b-12d3-a456-426614174000".parse().unwrap()].into_iter().collect(),
            discoverable: Some(true),
            local_name: Some("le_advertise".to_string()),
            ..Default::default()
        };
        adapter.set_discoverable(true);
        adapter.set_pairable(true);
        adapter.set_powered(true);
        let handle = adapter.advertise(le_advertisement).await?;



        if all_properties {
            query_all_adapter_properties(&adapter).await?;
        } else {
            query_adapter(&adapter).await?;
        }
        println!();
    }
    println!("Press enter to quit");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;
    Ok(())
}
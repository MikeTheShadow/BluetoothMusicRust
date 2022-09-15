use std::env;

async fn main() -> bluer::Result<()> {
    let all_properties = env::args().any(|arg| arg == "--all-properties");

    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    for adapter_name in adapter_names {
        println!("Bluetooth adapter {}:", &adapter_name);
        let adapter = session.adapter(&adapter_name)?;
        if all_properties {
            query_all_adapter_properties(&adapter).await?;
        } else {
            query_adapter(&adapter).await?;
        }
        println!();
    }
    Ok(())
}
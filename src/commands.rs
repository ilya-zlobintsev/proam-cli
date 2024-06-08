use crate::{
    exporter,
    protocol::{
        device_info::{build_device_info, FlashlightMode},
        notification::{process_notification, StatsUpdate},
        request::request_to_buf,
    },
};
use anyhow::Context;
use btleplug::{
    api::{Central, CentralEvent, CharPropFlags, Peripheral as _, ScanFilter, WriteType},
    platform::{Adapter, Peripheral},
};
use futures::{stream, Stream, StreamExt};

pub async fn connect(adapter: &Adapter, device_name: &str) -> anyhow::Result<()> {
    println!("Scanning for devices...");

    let mut events = adapter.events().await?;
    adapter
        .start_scan(ScanFilter::default())
        .await
        .context("Could not start scan")?;

    while let Some(event) = events.next().await {
        if let CentralEvent::DeviceDiscovered(peripheral_id) = event {
            let peripheral = adapter.peripheral(&peripheral_id).await?;
            let properties = peripheral.properties().await?;

            if let Some(name) = properties.and_then(|props| props.local_name) {
                if name.contains(device_name) {
                    println!("Found device '{name}', connecting");
                    return connect_device(adapter, peripheral).await;
                }
            }
        }
    }
    todo!();
}

async fn connect_device(adapter: &Adapter, peripheral: Peripheral) -> anyhow::Result<()> {
    adapter.stop_scan().await?;
    peripheral.connect().await.context("Failed to connect")?;
    println!("Connected");

    Ok(())
}

pub async fn status(adapter: &Adapter, device_name: &str) -> anyhow::Result<()> {
    let peripheral = get_connected_device(adapter, device_name)
        .await?
        .context("Not connected to a device")?;

    let updates_stream = setup_stats_stream(peripheral).await?;

    let device_info = build_device_info(updates_stream)
        .await
        .context("Could not collect device info")?;

    println!("{device_info:#?}");
    Ok(())
}

async fn get_connected_device(
    adapter: &Adapter,
    device_name: &str,
) -> anyhow::Result<Option<Peripheral>> {
    let peripherals = adapter.peripherals().await?;

    for peripheral in peripherals {
        if let Some(properties) = peripheral.properties().await? {
            if let Some(name) = properties.local_name {
                if name.contains(device_name) {
                    return Ok(Some(peripheral));
                }
            }
        }
    }

    Ok(None)
}

async fn setup_stats_stream(
    peripheral: Peripheral,
) -> anyhow::Result<impl Stream<Item = StatsUpdate> + Unpin> {
    peripheral
        .discover_services()
        .await
        .context("Could not discover services")?;

    let notify_characteristic = peripheral
        .characteristics()
        .into_iter()
        .find(|characteristic| characteristic.properties.contains(CharPropFlags::NOTIFY))
        .context("Could not find a notify characteristic")?;

    peripheral
        .subscribe(&notify_characteristic)
        .await
        .context("Could not subscribe to characteristic")?;

    let notification_stream = peripheral.notifications().await?;

    let updates_stream = notification_stream
        .skip(1)
        .flat_map(|notification| stream::iter(process_notification(&notification.value)));
    Ok(updates_stream)
}

pub async fn exporter(adapter: &Adapter, device_name: &str, port: u16) -> anyhow::Result<()> {
    let device = get_connected_device(adapter, device_name)
        .await?
        .context("Not connected to a device")?;

    let stream = setup_stats_stream(device).await?;
    exporter::run(port, stream).await
}

pub async fn flashlight(
    adapter: &Adapter,
    device_name: &str,
    mode: Option<FlashlightMode>,
) -> anyhow::Result<()> {
    let peripheral = get_connected_device(adapter, device_name)
        .await?
        .context("Not connected to a device")?;

    match mode {
        Some(mode) => {
            peripheral
                .discover_services()
                .await
                .context("Could not discover services")?;
            let write_characteristic = peripheral
                .characteristics()
                .into_iter()
                .find(|characteristic| characteristic.properties.contains(CharPropFlags::WRITE))
                .context("Could not find a write characteristic")?;

            let request = [0x13, 0x01, 0x00, mode as u8];
            peripheral
                .write(
                    &write_characteristic,
                    &request_to_buf(&request),
                    WriteType::WithoutResponse,
                )
                .await?;

            println!("Set flashlight to {mode}");
        }
        None => {
            let mut stream = setup_stats_stream(peripheral).await?;
            while let Some(update) = stream.next().await {
                if let StatsUpdate::FlashlightStatus(mode) = update {
                    println!("Current flashlight mode is: {mode}");
                    break;
                }
            }
        }
    }
    Ok(())
}

use crate::protocol::notification::StatsUpdate;
use anyhow::Context;
use futures::Stream;
use futures::StreamExt;
use prometheus::labels;
use prometheus::opts;
use prometheus::register_int_gauge;
use prometheus::register_int_gauge_vec;

pub async fn run(
    port: u16,
    mut stream: impl Stream<Item = StatsUpdate> + Unpin,
) -> anyhow::Result<()> {
    let binding = format!("0.0.0.0:{port}")
        .parse()
        .context("Could not parse listen URL")?;
    println!("Exporter listening on '{binding}'");
    prometheus_exporter::start(binding)?;

    let battery_charge =
        register_int_gauge!("powerroam_battery_charge", "Battery charge level").unwrap();
    let charge_time =
        register_int_gauge!("powerroam_charge_time", "Battery charge time in minutes").unwrap();
    let discharge_time = register_int_gauge!(
        "powerroam_discharge_time",
        "Battery discharge time in minutes"
    )
    .unwrap();
    let total_input = register_int_gauge!("powerroam_total_input", "Total input power").unwrap();
    let total_output = register_int_gauge!("powerroam_total_output", "Total output power").unwrap();
    let ac_output = register_int_gauge!("powerroam_ac_output", "Current AC output").unwrap();
    let dc_output =
        register_int_gauge_vec!(opts!("powerroam_dc_output", "Current DC output"), &["type"])
            .unwrap();

    while let Some(update) = stream.next().await {
        use StatsUpdate::*;
        match update {
            AcPower(value) => ac_output.set(value.into()),
            DcPower(power) => {
                for (name, value) in [
                    ("total", power.total),
                    ("type_c_one", power.type_c_one_power),
                    ("type_c_two", power.type_c_two_power),
                    ("usb_one", power.usb_one_power),
                    ("usb_two", power.usb_two_power),
                ] {
                    dc_output
                        .with(&labels! {
                            "type" => name
                        })
                        .set(value.into());
                }
            }
            TotalPower(total) => {
                total_input.set(total.input.into());
                total_output.set(total.output.into());
            }
            Capacity(capacity) => {
                battery_charge.set(capacity.battery_capacity_power.into());

                let charge = if capacity.charge_time == u16::MAX {
                    0
                } else {
                    capacity.charge_time as i64
                };
                charge_time.set(charge);

                let discharge = if capacity.discharge_time == u16::MAX {
                    0
                } else {
                    capacity.discharge_time as i64
                };
                discharge_time.set(discharge);
            }
            _ => (),
        }
    }

    println!("Notification stream ended");

    Ok(())
}

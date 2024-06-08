use clap::Subcommand;
use futures::{Stream, StreamExt};
use strum::{Display, FromRepr};

use super::notification::StatsUpdate;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DeviceInfo {
    pub power: Power,
    pub total_power: TotalPower,
    pub ac_power: u16,
    pub flashlight: FlashlightMode,
    pub dc_power: DcPower,
    pub status: Status,
    pub electric_quantity_power: u8,
    pub capacity: CapacityInfo,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Power {
    pub batteries_one_power: u16,
    pub batteries_two_power: u16,
    pub inverter_one_power: u16,
    pub inverter_two_power: u16,
}

// impl DeviceInfo {
//     pub fn apply_update(&mut self, update: StatsUpdate) {
//         match update {
//             StatsUpdate::TotalPower(v) => self.total_power = v,
//             StatsUpdate::AcPower(v) => self.ac_power = v,
//             StatsUpdate::FlashlightStatus(v) => self.flashlight = v,
//             StatsUpdate::DcPower(v) => self.dc_power = v,
//             StatsUpdate::Status(v) => self.status = v,
//             StatsUpdate::ElectricQuantityPower(v) => self.electric_quantity_power = v,
//             StatsUpdate::Capacity(v) => self.capacity = v,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Clone, Copy, FromRepr, Default, Display, Subcommand)]
pub enum FlashlightMode {
    #[default]
    Off = 0,
    Low = 1,
    High = 2,
    Strobe = 3,
    #[strum(serialize = "SOS")]
    Sos = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TotalPower {
    pub input: u16,
    pub output: u16,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct DcPower {
    pub type_c_one_power: u16,
    pub type_c_two_power: u16,
    pub usb_one_power: u16,
    pub usb_two_power: u16,
    pub total: u16,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct Status {
    pub low_noise: bool,
    pub low_battery_warning: bool,
    pub usb_switch: bool,
    pub dc_switch: bool,
    pub ac_frequency_hz: u8,
    pub warning_voice: bool,
    pub ac_turbo: bool,
    pub ac_switch: bool,
    pub battery_health: bool,
    pub locking: bool,
    pub key_voice: bool,
    pub standby: bool,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct CapacityInfo {
    pub charge_time: u16,
    pub discharge_time: u16,
    pub battery_capacity_power: u8,
}

pub async fn build_device_info(
    mut updates_stream: impl Stream<Item = StatsUpdate> + Unpin,
) -> Option<DeviceInfo> {
    let mut power = None;
    let mut total_power = None;
    let mut ac_power = None;
    let mut flashlight = None;
    let mut dc_power = None;
    let mut status = None;
    let mut electric_quantity_power = None;
    let mut capacity = None;

    while let Some(update) = updates_stream.next().await {
        match update {
            StatsUpdate::Power(v) => power = Some(v),
            StatsUpdate::TotalPower(v) => total_power = Some(v),
            StatsUpdate::AcPower(v) => ac_power = Some(v),
            StatsUpdate::FlashlightStatus(v) => flashlight = Some(v),
            StatsUpdate::DcPower(v) => dc_power = Some(v),
            StatsUpdate::Status(v) => status = Some(v),
            StatsUpdate::ElectricQuantityPower(v) => electric_quantity_power = Some(v),
            StatsUpdate::Capacity(v) => capacity = Some(v),
        }

        if let (
            Some(power),
            Some(total_power),
            Some(ac_power),
            Some(flashlight),
            Some(dc_power),
            Some(status),
            Some(electric_quantity_power),
            Some(capacity),
        ) = (
            power,
            total_power,
            ac_power,
            flashlight,
            dc_power,
            status,
            electric_quantity_power,
            capacity,
        ) {
            return Some(DeviceInfo {
                power,
                total_power,
                ac_power,
                flashlight,
                dc_power,
                status,
                electric_quantity_power,
                capacity,
            });
        }
    }

    None
}

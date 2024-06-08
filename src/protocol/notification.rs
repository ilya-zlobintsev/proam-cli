use crc16::{State, MODBUS};

use super::{
    device_info::{CapacityInfo, DcPower, FlashlightMode, Power, Status, TotalPower},
    SEPARATOR_SEQUENCE,
};

#[derive(Debug, PartialEq, Eq)]
pub enum StatsUpdate {
    Power(Power),
    TotalPower(TotalPower),
    AcPower(u16),
    FlashlightStatus(FlashlightMode),
    DcPower(DcPower),
    Status(Status),
    ElectricQuantityPower(u8),
    Capacity(CapacityInfo),
}

pub fn process_notification(data: &[u8]) -> Vec<StatsUpdate> {
    let mut updates = Vec::with_capacity(2);

    let mut start = 0;
    for i in 0..data.len() - 4 {
        if data[i..i + 4] == SEPARATOR_SEQUENCE {
            let value = &data[start..i];
            updates.extend(process_value(value));
            start = i;
        }
    }
    updates.extend(process_value(&data[start..]));
    updates
}

fn process_value(value: &[u8]) -> Option<StatsUpdate> {
    if value.is_empty() {
        return None;
    }

    let key = value[4];
    let end_offset = parse_u16(&value[5..7]) as usize;

    let payload = &value[7..end_offset + 9];
    let to_validate = &value[2..end_offset + 7];
    let (value, checksum_buf) = payload.split_at(payload.len() - 2);

    let expected_checksum = parse_u16(checksum_buf);
    let calculated_checksum = State::<MODBUS>::calculate(to_validate);

    if calculated_checksum != expected_checksum {
        eprintln!(
            "Checksum validation not passed, payload {:?}",
            value
                .iter()
                .map(|item| format!("{item:x}"))
                .collect::<Vec<_>>()
                .join("")
        );
        return None;
    }

    match key {
        0x04 if value.len() >= 8 => Some(StatsUpdate::Power(Power {
            batteries_one_power: parse_u16(&value[0..2]),
            batteries_two_power: parse_u16(&value[2..4]),
            inverter_one_power: parse_u16(&value[4..6]),
            inverter_two_power: parse_u16(&value[6..8]),
        })),
        0x09 if value.len() >= 24 => Some(StatsUpdate::Capacity(CapacityInfo {
            charge_time: parse_u16(&value[19..21]),
            discharge_time: parse_u16(&value[21..23]),
            battery_capacity_power: value[23],
        })),
        0x0b if value.len() >= 8 => {
            let buf = &value[6..8];
            let ac_power = u16::from_le_bytes(buf.try_into().unwrap());
            Some(StatsUpdate::AcPower(ac_power))
        }
        0x0c if value.len() >= 10 => Some(StatsUpdate::DcPower(DcPower {
            type_c_one_power: parse_u16(&value[0..2]),
            type_c_two_power: parse_u16(&value[2..4]),
            usb_one_power: parse_u16(&value[4..6]),
            usb_two_power: parse_u16(&value[6..8]),
            total: parse_u16(&value[8..10]),
        })),
        0x0f if value.len() >= 4 => Some(StatsUpdate::TotalPower(TotalPower {
            input: parse_u16(&value[0..2]),
            output: parse_u16(&value[2..4]),
        })),
        0x13 if !value.is_empty() => {
            let mode = FlashlightMode::from_repr(value[0] as usize).unwrap();
            Some(StatsUpdate::FlashlightStatus(mode))
        }
        0x15 if !value.is_empty() => Some(StatsUpdate::ElectricQuantityPower(value[0])),
        0x16 if value.len() >= 12 => Some(StatsUpdate::Status(Status {
            low_noise: value[0] != 0,
            low_battery_warning: value[1] != 0,
            usb_switch: value[2] != 0,
            dc_switch: value[3] != 0,
            ac_frequency_hz: value[4],
            warning_voice: value[5] != 0,
            ac_turbo: value[6] != 0,
            ac_switch: value[7] != 0,
            battery_health: value[8] != 0,
            locking: value[9] != 0,
            key_voice: value[10] == 0,
            standby: value[11] != 0,
        })),
        _ => None,
    }
}

fn parse_u16(data: &[u8]) -> u16 {
    u16::from_le_bytes(data.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{process_notification, StatsUpdate};
    use crate::protocol::device_info::{
        CapacityInfo, DcPower, FlashlightMode, Power, Status, TotalPower,
    };
    use pretty_assertions::assert_eq;

    fn assert_stats(data: &[u8], expected_updates: &[StatsUpdate]) {
        let updates = process_notification(data);
        assert_eq!(expected_updates, updates);
    }

    #[test]
    fn message_1() {
        let data = [
            0x5a, 0xa5, 0xc0, 0xa1, 0x0f, 0x04, 0x00, 0x00, 0x00, 0x03, 0x00, 0x6f, 0x35, 0x5a,
            0xa5, 0xc0, 0xa1, 0x0b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xd8, 0x26, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_stats(
            &data,
            &[
                StatsUpdate::TotalPower(TotalPower {
                    input: 0,
                    output: 3,
                }),
                StatsUpdate::AcPower(0),
            ],
        );
    }

    #[test]
    fn message_2() {
        let data = [
            0x5a, 0xa5, 0xc0, 0xa1, 0x16, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x00, 0x00, 0x9e, 0x0c, 0x5a, 0xa5, 0xc0, 0xa1, 0x13, 0x01, 0x00,
            0x01, 0x38, 0x46, 0x5a, 0xa5, 0xc0, 0xa1, 0x17, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xc6, 0x7a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_stats(
            &data,
            &[
                StatsUpdate::Status(Status {
                    low_noise: true,
                    low_battery_warning: false,
                    usb_switch: true,
                    dc_switch: true,
                    ac_frequency_hz: 0,
                    warning_voice: false,
                    ac_turbo: false,
                    ac_switch: true,
                    battery_health: true,
                    locking: false,
                    key_voice: true,
                    standby: false,
                }),
                StatsUpdate::FlashlightStatus(FlashlightMode::Low),
            ],
        );
    }

    #[test]
    fn message_3() {
        let data = [
            0x5a, 0xa5, 0xc0, 0xa1, 0x16, 0x0c, 0x00, 0x01, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00,
            0x01, 0x01, 0x00, 0x00, 0x00, 0x9e, 0x0c, 0x5a, 0xa5, 0xc0, 0xa1, 0x13, 0x01, 0x00,
            0x00, 0xf9, 0x86, 0x5a, 0xa5, 0xc0, 0xa1, 0x17, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xc6, 0x7a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_stats(
            &data,
            &[
                StatsUpdate::Status(Status {
                    low_noise: true,
                    low_battery_warning: false,
                    usb_switch: true,
                    dc_switch: true,
                    ac_frequency_hz: 0,
                    warning_voice: false,
                    ac_turbo: false,
                    ac_switch: true,
                    battery_health: true,
                    locking: false,
                    key_voice: true,
                    standby: false,
                }),
                StatsUpdate::FlashlightStatus(FlashlightMode::Off),
            ],
        );
    }

    #[test]
    fn message_4() {
        let data = [
            0x5a, 0xa5, 0xc0, 0xa1, 0x0c, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6a, 0x99, 0x5a, 0xa5, 0xc0, 0xa1, 0x15,
            0x01, 0x00, 0x00, 0xf9, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_stats(
            &data,
            &[
                StatsUpdate::DcPower(DcPower {
                    type_c_one_power: 0,
                    type_c_two_power: 0,
                    usb_one_power: 0,
                    usb_two_power: 3,
                    total: 0,
                }),
                StatsUpdate::ElectricQuantityPower(0),
            ],
        );
    }

    #[test]
    fn message_5() {
        let data = [
            0x5a, 0xa5, 0xc0, 0xa1, 0x04, 0x08, 0x00, 0x44, 0x00, 0x47, 0x00, 0x55, 0x00, 0x4d,
            0x00, 0xc6, 0x1a, 0x5a, 0xa5, 0xc0, 0xa1, 0x09, 0x1c, 0x00, 0xf2, 0x00, 0x7e, 0x0d,
            0x93, 0x0d, 0x93, 0x0d, 0x8e, 0x0d, 0x90, 0x0d, 0x92, 0x0d, 0x92, 0x0d, 0x64, 0x09,
            0x00, 0x00, 0x00, 0xd4, 0x15, 0x5a, 0xa4, 0x6a, 0x00, 0x00, 0x10, 0xfc, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_stats(
            &data,
            &[
                StatsUpdate::Power(Power {
                    batteries_one_power: 68,
                    batteries_two_power: 71,
                    inverter_one_power: 85,
                    inverter_two_power: 77,
                }),
                StatsUpdate::Capacity(CapacityInfo {
                    charge_time: 0,
                    discharge_time: 5588,
                    battery_capacity_power: 90,
                }),
            ],
        );
    }
}

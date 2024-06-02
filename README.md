# Description

This is a CLI utility and Prometheus exporter for Ugreen PowerRoam portable power stations. It lets you connect to the power station over Bluetooth and view information, or export it as metrics to Prometheus.
Only the PowerRoam 600 was tested, but other models should work too.

# Building

This has only been tested on Linux, but other systems should work too.

Requirements:
- Rust
- pkg-config
- lidbdus-1-dev (exact name depends on distro)

To build the binary, run:
```
cargo build --release
```
Binary will be at `target/release/proam-cli`

# Usage

```
proam-cli connect
proam-cli status
```

To run the exporter:
```
proam-cli exporter
```

Example metrics endpoint output:
```
# HELP powerroam_ac_output Current AC output
# TYPE powerroam_ac_output gauge
powerroam_ac_output 0
# HELP powerroam_battery_charge Battery charge level
# TYPE powerroam_battery_charge gauge
powerroam_battery_charge 100
# HELP powerroam_charge_time Battery charge time in minutes
# TYPE powerroam_charge_time gauge
powerroam_charge_time 0
# HELP powerroam_discharge_time Battery discharge time in minutes
# TYPE powerroam_discharge_time gauge
powerroam_discharge_time 19113
# HELP powerroam_total_input Total input power
# TYPE powerroam_total_input gauge
powerroam_total_input 0
# HELP powerroam_total_output Total output power
# TYPE powerroam_total_output gauge
powerroam_total_output 0
```

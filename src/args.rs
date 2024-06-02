use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Command,

    /// Filter for the name of the bluetooth device
    #[arg(short, long, default_value = "ugreen gs")]
    pub device_name: String,
}

#[derive(Subcommand)]
pub enum Command {
    Status,
    Connect,
    Exporter {
        #[arg(short, long, default_value_t = 9091)]
        port: u16,
    },
}

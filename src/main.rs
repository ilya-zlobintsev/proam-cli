mod args;
mod commands;
mod exporter;
mod protocol;

use anyhow::Context;
use args::Args;
use btleplug::{
    api::Manager as _,
    platform::{Adapter, Manager},
};
use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let adapter = init_adapter()
        .await
        .context("Could not initialize bluetooth")?;

    match args.cmd {
        args::Command::Status => commands::status(&adapter, &args.device_name).await,
        args::Command::Connect => commands::connect(&adapter, &args.device_name).await,
        args::Command::Exporter { port } => {
            commands::exporter(&adapter, &args.device_name, port).await
        }
        args::Command::Flashlight { mode } => {
            commands::flashlight(&adapter, &args.device_name, mode).await
        }
    }
}

async fn init_adapter() -> anyhow::Result<Adapter> {
    let manager = Manager::new()
        .await
        .context("Could not initialize manager")?;
    let adapters = manager
        .adapters()
        .await
        .context("Could not fetch adapters")?;

    adapters.into_iter().next().context("No adapters found")
}

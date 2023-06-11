use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::info;

use crate::run::{check_commands, enable_services, init_cluster};
mod config;
mod helm;
mod run;

#[derive(Parser)]
#[command(version, about)]
struct KanopyCli {
    #[clap(subcommand)]
    subcmd: SubCommand,

    /// Specify config file location
    #[clap(
        short,
        long,
        env = "KANOPY_CONFIG",
        default_value = "/etc/kanopy/kanopy.yaml"
    )]
    config: String,

    /// Specify assets directory
    #[clap(short, long, env = "KANOPY_ASSETS", default_value = "/var/lib/kanopy")]
    assets: String,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Initialize Cluster
    #[clap(name = "init")]
    Init,
    /// Run first boot tasks
    #[clap(name = "firstboot")]
    FirstBoot,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    // install tracing_subscriber logging
    tracing_subscriber::fmt::init();
    let cli = KanopyCli::parse();

    match cli.subcmd {
        SubCommand::Init => {
            check_commands()?;
            // enable_services()?;
            // init_cluster()?;
        }
        SubCommand::FirstBoot => {
            // do something here that would be run on first boot
            check_commands()?;
            enable_services()?;
            init_cluster()?;
        }
    }

    Ok(())
}

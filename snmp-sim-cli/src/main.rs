mod cli;
mod cli_error;
mod operations;
mod subcommands;

pub use cli_error::*;

use crate::cli::SnmpSimCli;
use clap::Parser;
use color_eyre::{eyre::eyre, Help};
use human_panic::{handle_dump, Metadata};
use std::panic::set_hook;
use tracing::{self, debug};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().map_err(CliError::Eyre)?;

    // We are falling back to printing all spans at info-level or above
    // if the RUST_LOG environment variable has not been set.
    if let Ok(env_filter) = std::env::var("RUST_LOG") {
        let filter = EnvFilter::try_new(env_filter)
            .map_err(|e| {
                eyre!("Failed to parse the logging filter specified with RUST_LOG: {}", e).suggestion(
                    "Please try another filter.\
                      Example: RUST_LOG=snmp_sim_cli=debug",
                )
            })
            .map_err(CliError::Eyre)?;

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_thread_names(true)
            .with_ansi(false);

        tracing_subscriber::fmt::init();
    };

    debug!("Starting SNMP Simulator CLI...");

    // We register our own panic hook to customise the error message displayed,
    // the default error message generated by 'human_panic' is not exactly what we desire.
    set_hook(Box::new(|panic_info| {
        let metadata = Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: env!("CARGO_PKG_AUTHORS").into(),
            homepage: env!("CARGO_PKG_REPOSITORY").into(),
        };

        let mut error_msg = String::new();
        error_msg.push_str("The Safe CLI had a problem and crashed. To help us diagnose the problem you can send us a crash report.\n\n");
        match handle_dump(&metadata, panic_info) {
            Some(report_filepath) => error_msg.push_str(&format!("We have generated a report file at \"{}\".\nPlease submit an issue, including the report as an attachment, at {}.\n", report_filepath.display(), env!("CARGO_PKG_REPOSITORY"))),
            None => error_msg.push_str(&format!("Please submit an issue, including details to reproduce it, at {}.\n", env!("CARGO_PKG_REPOSITORY"))),
        }
        error_msg
            .push_str("In order to improve the software, we rely on people to submit reports.\n\nThank you kindly!\n");
        eprintln!("{}", error_msg);
    }));

    SnmpSimCli::parse().run().await
}

//! # Provides a mock Ambi client that emulates real sensor hardware such as
//! an Edge client.
//!
//! This application emulates a real set of hardware sensors that can report on
//! environmental conditions such as temperature, pressure, humidity, etc.
//!
//! Please see the `ambi` repository for the web backend that this client connects to
//! and the `edge-rs` repository for what this client is emulating.
//!
//! See the `LICENSE` file for Copyright and license details.
//!

use ambi_mock_client::error::PostSchedulerError;
use clap::Parser;
use log::LevelFilter;

fn main() -> Result<(), PostSchedulerError> {
    // Parses the provided command line interface arguments and flags
    let cli = ambi_mock_client::Cli::parse();

    init_logging(cli.debug);

    ambi_mock_client::run(&cli)
}

fn init_logging(is_debug: bool) {
    let mut logger_builder = env_logger::Builder::new();

    match is_debug {
        true => logger_builder.filter_level(LevelFilter::Debug),
        false => logger_builder.filter_level(LevelFilter::Info),
    };

    // The format_target is what makes the logs include `ambi_mock_client` everywhere. I left it
    // here in case anyone wants to disable it in the future. It is enabled because not all logs
    // originate from this crate (reqwest for instance makes a few as well).
    logger_builder.format_target(true).init();
}

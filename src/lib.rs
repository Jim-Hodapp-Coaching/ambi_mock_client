//! # Provides a mock Ambi client that emulates real sensor hardware such as like
//! an Edge client.
//!
//! This file provides for a separation of concerns from main.rs for application
//! logic, per the standard Rust pattern.
//!
//! See `main.rs` for more details about what this application does.
//!
//! See the `LICENSE` file for Copyright and license details.
//!

mod data;
mod error;
mod requests;

use crate::requests::{send_data, RequestSchedulerBuilder};
use clap::Parser;
use error::RequestSchedulerError;
use log::debug;
use std::time::Duration;

const URL: &str = "http://localhost:8000/api/readings/add";

/// Defines the Ambi Mock Client command line interface as a struct
#[derive(Parser, Debug)]
#[command(
    name = "Ambi Mock Client",
    author,
    version,
    about,
    long_about = "This application emulates a real set of hardware sensors that can report on environmental conditions such as temperature, pressure, humidity, etc."
)]
pub struct Cli {
    /// Turns verbose console debug output on
    #[arg(short, long)]
    pub debug: bool,
    /// The amount of requests to make
    /// [DEFAULT: 1]
    #[arg(short = 'n', long)]
    pub request_amount: Option<u32>,
    /// The time between each request (in seconds)
    /// [DEFAULT: 10]
    #[arg(short = 't', long = "time-per-request")]
    pub time_per_request_s: Option<u64>,
    /// The total time over which all the requests must be sent (in seconds, alternative to -t)
    #[arg(short = 'T', long = "total-time")]
    pub total_time_s: Option<u64>,
    /// The number of threads to spawn. The workload will be cloned to each thread, not divided
    /// [DEFAULT: 1]
    #[arg(short = 'p', long)]
    pub num_threads: Option<u32>,
}

pub fn run(cli: &Cli) -> Result<(), RequestSchedulerError> {
    debug!("cli: {cli:?}");

    let req_scheduler = RequestSchedulerBuilder::default()
        .with_some_request_amount(&cli.request_amount)
        .with_some_time_per_request(&cli.time_per_request_s.map(Duration::from_secs))
        .with_some_total_time(&cli.total_time_s.map(Duration::from_secs))
        .with_some_num_threads(&cli.num_threads)
        .build()?;

    send_data(req_scheduler);

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use crate::Cli;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}

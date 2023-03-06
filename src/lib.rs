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
pub mod error;
mod sensor_posts;

use crate::sensor_posts::{send_data, PostSchedulerBuilder};
use clap::Parser;
use error::PostSchedulerError;
use log::debug;
use sensor_posts::MAX_NUM_THREADS;
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
    /// Turns verbose console debug output on.
    #[arg(short, long)]
    pub debug: bool,
    /// The number of sensor readings to post.
    /// [DEFAULT: 1]
    #[arg(short = 'n', long)]
    pub post_amount: Option<u32>,
    /// The time between each sensor reading post (in seconds).
    /// [DEFAULT: 10]
    #[arg(short = 't', long = "time-per-post")]
    pub time_per_post_s: Option<u64>,
    /// The total time over which all the sensor reading posts must be sent (in seconds, alternative to -t).
    #[arg(short = 'T', long = "total-time")]
    pub total_time_s: Option<u64>,
    /// The number of threads to spawn. The workload will be cloned to each thread, not divided.
    /// [DEFAULT: 1]
    #[arg(short = 'p', long, value_parser = is_valid_num_of_threads)]
    pub num_threads: Option<u32>,
}

/// Ensures that the number of threads are in [1, `MAX_NUM_THREADS`].
fn is_valid_num_of_threads(val: &str) -> Result<u32, String> {
    match val.parse::<u32>() {
        Ok(num_threads) => match num_threads {
            0 => Err("You must use at least 1 thread.".to_string()),
            1..=MAX_NUM_THREADS => Ok(num_threads),
            _ => Err(format!(
                "You can't use more than {MAX_NUM_THREADS} threads."
            )),
        },
        Err(_) => Err(String::from("value wasn't a number!")),
    }
}

pub fn run(cli: &Cli) -> Result<(), PostSchedulerError> {
    debug!("cli: {cli:?}");

    let req_scheduler = PostSchedulerBuilder::default()
        .with_some_post_amount(&cli.post_amount)
        .with_some_time_per_post(&cli.time_per_post_s.map(Duration::from_secs))
        .with_some_total_time(&cli.total_time_s.map(Duration::from_secs))
        .with_some_num_threads(&cli.num_threads)
        .build()?;

    send_data(req_scheduler);

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::{error::ErrorKind, CommandFactory, Parser};

    use crate::{sensor_posts::MAX_NUM_THREADS, Cli};

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    // #[should_panic]
    fn test_invalid_num_threads_low() {
        let cli = Cli::try_parse_from(vec!["", "-p", "0"]);
        assert!(cli.is_err());

        let cli_err = cli.unwrap_err();
        assert_eq!(cli_err.kind(), ErrorKind::ValueValidation);
    }

    #[test]
    fn test_invalid_num_threads_high() {
        let num = MAX_NUM_THREADS + 1;
        let cli = Cli::try_parse_from(vec!["", "-p", &num.to_string()]);
        assert!(cli.is_err());

        let cli_err = cli.unwrap_err();
        assert_eq!(cli_err.kind(), ErrorKind::ValueValidation);
    }
}

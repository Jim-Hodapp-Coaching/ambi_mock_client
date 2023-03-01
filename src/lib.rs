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

use std::time::Duration;

use crate::data::{
    random_gen_dust_concentration, random_gen_humidity, random_gen_pressure, random_gen_temperature,
};
use crate::{
    data::Reading,
    requests::{send_data, RequestSchedulerBuilder},
};
use clap::Parser;
use error::RequestSchedulerError;
use log::debug;

use crate::data::AirPurity;

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
    #[arg(short = 'n', long)]
    pub request_amount: Option<u32>,
    #[arg(short = 't', long = "time-per-request")]
    pub time_per_request_s: Option<u64>,
    #[arg(short = 'T', long = "total-time")]
    pub total_time_s: Option<u64>,
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

    let dust_concentration = random_gen_dust_concentration();
    let air_purity = AirPurity::from_value(dust_concentration).to_string();
    let reading = Reading::new(
        random_gen_temperature(),
        random_gen_humidity(),
        random_gen_pressure(),
        dust_concentration,
        air_purity,
    );

    let json = serde_json::to_string(&reading).unwrap();

    send_data(req_scheduler, json, cli.debug);

    Ok(())

    // TODO: Move these logs to the requests file
    // info!("Sending POST request to {}", URL);
    // debug!("Request JSON: {}", json);
    //
    // let client = Client::new();
    // let res = client
    //     .post(URL)
    //     .header(CONTENT_TYPE, "application/json")
    //     .body(json)
    //     .send();

    // match res {
    //     Ok(response) => {
    //         info!("Response from Ambi backend: {}", response.status().as_str());
    //         debug!("Response from Ambi backend: {:#?}", response);
    //     }
    //     Err(e) => {
    //         if e.is_request() {
    //             error!("Response error from Ambi backend: request error");
    //         } else if e.is_timeout() {
    //             error!("Response error from Ambi backend: request timed out");
    //         } else {
    //             error!("Response error from Ambi backend: specific error type unknown");
    //         }

    //         debug!("{}", e.to_string());
    //         debug!("Response error from Ambi backend: {:?}", e);
    //     }
    // }
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

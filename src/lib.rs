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

use crate::data::Reading;
use crate::data::{
    random_gen_dust_concentration, random_gen_humidity, random_gen_pressure, random_gen_temperature,
};
use clap::Parser;
use log::{debug, error, info};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

use crate::data::AirPurity;

// TODO: Make the port configurable
const URL: &str = "http://localhost:4000/api/readings/add";

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
    #[clap(short, long)]
    pub debug: bool,
}

pub fn run(cli: &Cli) {
    debug!("cli: {cli:?}");

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

    info!("Sending POST request to {}", URL);
    debug!("Request JSON: {}", json);

    let client = Client::new();
    let res = client
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send();

    match res {
        Ok(response) => {
            info!(
                "Response from Ambi backend: {:?}",
                response.status().as_str()
            );
            debug!("Response from Ambi backend: {:#?}", response);
        }
        Err(e) => {
            if e.is_request() {
                error!("Response error from Ambi backend: request error");
            } else if e.is_timeout() {
                error!("Response error from Ambi backend: request timed out");
            } else {
                error!("Response error from Ambi backend: specific error type unknown");
            }

            debug!("{}", e.to_string());
            debug!("Response error from Ambi backend: {:?}", e);
        }
    }
}

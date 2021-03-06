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

use rand::{thread_rng, Rng};
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize, Deserialize};
use std::fmt;
use clap::{Parser};

/// Defines the Ambi Mock Client command line interface as a struct
#[derive(Parser, Debug)]
#[clap(name = "Ambi Mock Client")]
#[clap(author = "Rust Never Sleeps community (https://github.com/Jim-Hodapp-Coaching/)")]
#[clap(version = "0.1.0")]
#[clap(about = "Provides a mock Ambi client that emulates real sensor hardware such as an Edge client.")]
#[clap(long_about = "This application emulates a real set of hardware sensors that can report on environmental conditions such as temperature, pressure, humidity, etc.")]
pub struct Cli {
    /// Turns verbose console debug output on
    #[clap(short, long)]
    pub debug: bool,
}


#[derive(Serialize, Deserialize)]
struct Reading {
  temperature: String,
  humidity: String,
  pressure: String,
  dust_concentration: String,
  air_purity: String
}

impl Reading {
    fn new(
        temperature: String,
        humidity: String,
        pressure: String,
        dust_concentration: String,
        air_purity: String
    ) -> Reading {
        Reading {
            temperature,
            humidity,
            pressure,
            dust_concentration,
            air_purity
        }
    }
}

#[derive(Debug, PartialEq)]
enum AirPurity {
    Dangerous,
    High,
    Low,
    FreshAir
}

impl AirPurity {
    fn from_value(value: u16) -> AirPurity {
        match value {
            0..=50 => return AirPurity::FreshAir,
            51..=100 => return AirPurity::Low,
            101..=150 => return AirPurity::High,
            151.. => return AirPurity::Dangerous,
        };
    }
}

// implements fmt::Display for AirPurity so that we can call .to_string() on
// each enum value
impl fmt::Display for AirPurity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AirPurity::Low => write!(f, "Fresh Air"),
            AirPurity::High => write!(f, "Low Pollution"),
            AirPurity::Dangerous => write!(f, "High Pollution"),
            AirPurity::FreshAir => write!(f, "Dangerous Pollution")
        }
    }
}

fn random_gen_humidity() -> String {
    let mut rng = thread_rng();
    let value = rng.gen_range(0.0..=100.0);
    format!("{:.1}", value)
}

fn random_gen_temperature() -> String {
    let mut rng = thread_rng();
    let value = rng.gen_range(15.0..=35.0);
    format!("{:.1}", value)
}

fn random_gen_pressure() -> String {
    let mut rng = thread_rng();
    rng.gen_range(900..=1100).to_string()
}

fn random_gen_dust_concentration() -> String {
    let mut rng = thread_rng();
    rng.gen_range(0..=1000).to_string()
}

pub fn run(cli: &Cli) {
    println!("\r\ncli: {:?}\r\n", cli);

    let dust_concentration = random_gen_dust_concentration();
    let air_purity = AirPurity::from_value(dust_concentration.parse::<u16>().unwrap()).to_string();
    let reading = Reading::new(
        random_gen_temperature(),
        random_gen_humidity(),
        random_gen_pressure(),
        dust_concentration,
        air_purity
    );

    let json = serde_json::to_string(&reading).unwrap();
    const URL: &str = "http://localhost:4000/api/readings/add";

    println!("Sending POST request to {} as JSON: {}", URL, json);

    let client = Client::new();
    let res = client
        .post(URL)
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send();

    match res {
        Ok(response) => {
            match cli.debug {
                true => println!("Response from Ambi backend: {:#?}", response),
                false => println!("Response from Ambi backend: {:?}", response.status().as_str())
            }
        }
        Err(e) => {
            match cli.debug {
                // Print out the entire reqwest::Error for verbose debugging
                true => eprintln!("Response error from Ambi backend: {:?}", e),
                // Keep the error reports more succinct
                false => {
                    if e.is_request() {
                        eprintln!("Response error from Ambi backend: request error");
                    } else if e.is_timeout() {
                        eprintln!("Response error from Ambi backend: request timed out");
                    } else {
                        eprintln!("Response error from Ambi backend: specific error type unknown");
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn random_gen_humidity_returns_correctly_formatted_humidity_data() {
      let result = random_gen_humidity();
      let regex = Regex::new(r"\d{1,2}.\d{1}").unwrap();

      assert!(regex.is_match(&result));
    }
    
    #[test]
    fn random_gen_temperature_returns_correctly_formatted_humidity_data() {
      let result = random_gen_temperature();
      let regex = Regex::new(r"\d{1,2}.\d{1}").unwrap();

      assert!(regex.is_match(&result));
    }

    #[test]
    fn random_gen_pressure_returns_correctly_formatted_pressure_data() {
        let result = random_gen_pressure();
        let regex = Regex::new(r"\d{3,4}").unwrap();
        assert!(regex.is_match(&result));
    }

    #[test]
    fn random_gen_dust_concentration_returns_correctly_formatted_pressure_data() {
        let result = random_gen_dust_concentration();
        let regex = Regex::new(r"\d{0,4}").unwrap();
        assert!(regex.is_match(&result));
    }

    #[test]
    fn air_purity_from_value_returns_correct_enum() {
        let mut rng = thread_rng();
        let fresh_air = rng.gen_range(0..=50);
        let low = rng.gen_range(51..=100);
        let high = rng.gen_range(101..=150);
        let dangerous = rng.gen_range(151..u16::MAX);

        assert_eq!(AirPurity::from_value(fresh_air), AirPurity::FreshAir);
        assert_eq!(AirPurity::from_value(low), AirPurity::Low);
        assert_eq!(AirPurity::from_value(high), AirPurity::High);
        assert_eq!(AirPurity::from_value(dangerous), AirPurity::Dangerous);
    }
}

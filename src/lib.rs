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
use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize, Deserialize};
use std::fmt;
use clap::{Parser};
use std::thread::{spawn, JoinHandle, ThreadId};

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

    /// Make <INT> number of concurrent requests
    #[clap(short, long, default_value_t = 1)]
    pub int: u16
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

#[derive(Debug)]
struct Output {
    description: String,
    error: Option<Error>,
    data: Option<Response>,
    thread_id: ThreadId,
    debug: bool
  }

impl Output {
    pub fn new(
        description: String,
        error: Option<Error>,
        data: Option<Response>,
        thread_id: ThreadId,
        debug: bool
    ) -> Output {
        Output {
            description,
            error,
            data,
            thread_id,
            debug
        }
    }

    pub fn is_error(&self) -> bool {
      self.error.is_some()
    }

    pub fn print(&self) {
        if self.is_error() {
           self.print_to_stderr()     
        } else {
          self.print_to_stdout()
        }
    }

    fn print_to_stderr(&self) {
      if self.debug {
        eprintln!("{:#?}", self)
      } else {
        eprintln!("{}", self)
      }
    }

    fn print_to_stdout(&self) {
        if self.debug {
          println!("{:#?}", self)
        } else {
          println!("{}", self)
        }
      }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_error() {
            let error = self.error.as_ref().unwrap();
            write!(f, "{} Status: {:?}, Thread ID: {:?}", self.description, error.inner.status(), self.thread_id)
        } else {
            let response = self.data.as_ref().unwrap();
            let status = response.status().as_u16();
            write!(f, "{} Status: {}, Thread ID: {:?}", self.description, status, self.thread_id)
        }
    }
}

#[derive(Debug)]
struct Error {
  inner: reqwest::Error
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error {
            inner: error
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

fn send_request(url: &str, client: Client) -> reqwest::Result<Response> {
    let dust_concentration = random_gen_dust_concentration();
    let air_purity = AirPurity::from_value(dust_concentration.parse::<u16>().unwrap()).to_string();
    let reading = Reading::new(
        random_gen_temperature(),
        random_gen_humidity(),
        random_gen_pressure(),
        dust_concentration,
        air_purity,
    );

    let json = serde_json::to_string(&reading).unwrap();
    println!("Sending POST request to {} as JSON: {}", url, json);
    client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send()
}

pub fn run(cli: &Cli) {
    println!("\r\ncli: {:?}\r\n", cli);

    const URL: &str = "http://localhost:4000/api/readings/add";
    let mut handlers: Vec<JoinHandle<()>> = vec![];
    let debug: bool = cli.debug;
    for _ in 0..cli.int {
        let handler = spawn(move ||
            match send_request(URL, Client::new()) {
              Ok(response) => {
                Output::new(
                   String::from("Response from Ambi backend."),
                   None,
                   Some(response),
                   std::thread::current().id(),
                   debug   
                ).print(); 
              }
              Err(error) => {
                Output::new(
                    String::from("Response error from Ambi backend."),
                    Some(error.into()),
                    None,
                    std::thread::current().id(),
                    debug   
                 ).print();
              }
            }
        );

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
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

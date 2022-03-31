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
//! 
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

pub fn run(cli: &Cli) {
    println!("\r\ncli: {:?}\r\n", cli);
}
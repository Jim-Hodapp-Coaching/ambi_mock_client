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

use clap::Parser;

// Internal library namespace for separation of app logic
use ambi_mock_client;

fn main() {
    // Parses the provided command line interface arguments and flags
    let cli = ambi_mock_client::Cli::parse();

    ambi_mock_client::run(&cli);
}

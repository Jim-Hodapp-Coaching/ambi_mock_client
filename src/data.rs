use core::fmt;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Reading {
    temperature: f64,
    humidity: f64,
    pressure: i32,
    dust_concentration: f64,
    air_purity: String,
}

impl Reading {
    pub(crate) fn new(
        temperature: f64,
        humidity: f64,
        pressure: i32,
        dust_concentration: f64,
        air_purity: String,
    ) -> Reading {
        Reading {
            temperature,
            humidity,
            pressure,
            dust_concentration,
            air_purity,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum AirPurity {
    Dangerous,
    High,
    Low,
    FreshAir,
}

impl AirPurity {
    pub(crate) fn from_value(value: f64) -> AirPurity {
        match value {
            value if (f64::MIN..=50.0).contains(&value) => AirPurity::FreshAir,
            value if value > 50.0 && value <= 100.0 => AirPurity::Low,
            value if value > 100.0 && value <= 150.0 => AirPurity::High,
            _ => AirPurity::Dangerous,
        }
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
            AirPurity::FreshAir => write!(f, "Dangerous Pollution"),
        }
    }
}

pub(crate) fn random_gen_humidity() -> f64 {
    let mut rng = thread_rng();
    let value = rng.gen_range(0.0..=100.0);
    // Limit to 2 decimals
    f64::trunc(value * 100.0) / 100.0
}

pub(crate) fn random_gen_temperature() -> f64 {
    let mut rng = thread_rng();
    let value = rng.gen_range(15.0..=35.0);
    // Limit to 2 decimals
    f64::trunc(value * 100.0) / 100.0
}

pub(crate) fn random_gen_pressure() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(900..=1100)
}

pub(crate) fn random_gen_dust_concentration() -> f64 {
    let mut rng = thread_rng();
    let value = rng.gen_range(0.0..=1000.0);
    f64::trunc(value * 100.0) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn air_purity_from_value_returns_correct_enum() {
        let mut rng = thread_rng();
        let fresh_air = rng.gen_range(0.0..=50.0);
        let low = rng.gen_range(51.0..=100.0);
        let high = rng.gen_range(101.0..=150.0);
        let dangerous = rng.gen_range(151.0..f64::MAX);

        assert_eq!(AirPurity::from_value(fresh_air), AirPurity::FreshAir);
        assert_eq!(AirPurity::from_value(low), AirPurity::Low);
        assert_eq!(AirPurity::from_value(high), AirPurity::High);
        assert_eq!(AirPurity::from_value(dangerous), AirPurity::Dangerous);
    }
}

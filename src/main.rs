use rand::{thread_rng, Rng};
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Reading {
  tempurature: String,
  humidity: String,
  pressure: String,
  dust_concentration: String,
  air_purity: String
}

impl Reading {
    fn new(
        tempurature: String,
        humidity: String,
        pressure: String,
        dust_concentration: String,
        air_purity: String
    ) -> Reading {
        Reading {
            tempurature: tempurature,
            humidity: humidity,
            pressure: pressure,
            dust_concentration: dust_concentration,
            air_purity: air_purity
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
            AirPurity::Low => write!(f, "LOW"),
            AirPurity::High => write!(f, "HIGH"),
            AirPurity::Dangerous => write!(f, "DANGEROUS"),
            AirPurity::FreshAir => write!(f, "FRESH_AIR")
        }
    }
}

fn random_gen_humidity() -> String {
    let mut rng = thread_rng();
    let value = rng.gen_range(0.0..=100.0);
    format!("{:.1}", value)
}

fn random_gen_tempurature() -> String {
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

fn main() {
   let dust_concentration = random_gen_dust_concentration();
   let air_purity = AirPurity::from_value(dust_concentration.parse::<u16>().unwrap()).to_string();
   let reading = Reading::new(
       random_gen_tempurature(),
       random_gen_humidity(),
       random_gen_pressure(),
       dust_concentration,
       air_purity
   );

   let json = serde_json::to_string(&reading).unwrap();
   const URL: &str = "http://localhost:4000/api/readings/add";

   println!("Sending POST request to {} as JSON: {}", URL, json);

   let client = Client::new();
   let res = client.post(URL)
    .body(json)
    .send();

   println!("Response: {:#?}", res);
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
    fn random_gen_tempurature_returns_correctly_formatted_humidity_data() {
      let result = random_gen_tempurature();
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

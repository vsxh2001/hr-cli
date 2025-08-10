use std::str::FromStr;

use serde::{Deserialize, Serialize};
use clap::{Args, arg};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Metric {
    pub name: String,
    pub value: u8,
}

impl FromStr for Metric {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: Parse "name:value" into a Metric
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid format. Expected 'name:value'.".to_string());
        }
        let name = parts[0].to_string();
        let value = parts[1]
            .parse::<u8>()
            .map_err(|_| "Value must be a valid number.".to_string())?;
        Ok(Metric { name, value })
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Args)]
pub struct Human {
    /// identification number
    #[arg(long)]
    pub id: Option<String>,

    /// name of the human
    #[arg(short, long)]
    pub name: String,

    /// phone number of the human
    #[arg(long)]
    pub phone: Option<String>,

    /// free-form description of the human
    #[arg(short, long)]
    pub description: Option<String>,

    /// labels associated with the human
    #[arg(long)]
    pub label: Option<Vec<String>>,

    /// metrics associated with the human
    #[arg(long, value_parser = clap::value_parser!(Metric))]
    pub metric: Option<Vec<Metric>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_from_str_valid() {
        let m: Metric = "speed:42".parse().expect("parse Metric");
        assert_eq!(m.name, "speed");
        assert_eq!(m.value, 42);
    }

    #[test]
    fn metric_from_str_invalid_format() {
        let err = "speed-42".parse::<Metric>().unwrap_err();
        assert!(err.contains("Invalid format"));
    }

    #[test]
    fn metric_from_str_invalid_value() {
        let err = "speed:NaN".parse::<Metric>().unwrap_err();
        assert!(err.contains("valid number"));
    }
}

// Test fixtures shared across unit tests in this crate
#[cfg(test)]
pub mod humans {
    use super::{Human, Metric};

    fn mk(name: &str, labels: &[&str], metrics: &[(&str, u8)], description: Option<&str>, id: Option<&str>, phone: Option<&str>) -> Human {
        Human {
            id: id.map(|s| s.to_string()),
            name: name.to_string(),
            phone: phone.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            label: if labels.is_empty() { None } else { Some(labels.iter().map(|s| s.to_string()).collect()) },
            metric: if metrics.is_empty() {
                None
            } else {
                Some(metrics.iter().map(|(n, v)| Metric { name: (*n).into(), value: *v }).collect())
            },
        }
    }

    /// Create a small, reusable set of Humans for tests.
    /// Contains: Jane (with id/phone/description), alice (eng,oncall), alina (eng), bob (sales)
    pub fn test_setup() -> Vec<Human> {
        vec![
            // Matches storage roundtrip expectations
            mk("Jane", &["eng", "team-a"], &[("speed", 7), ("height", 42)], Some("A description"), Some("123"), Some("555-0100")),
            // Used by search tests
            mk("alice", &["eng", "oncall"], &[("speed", 10), ("height", 20)], Some("team lead"), None, Some("555")),
            mk("alina", &["eng"], &[("speed", 11), ("height", 20)], None, None, None),
            mk("bob", &["sales"], &[("speed", 9)], Some("intern"), None, None),
        ]
    }
}
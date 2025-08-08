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

    /// labels associated with the human
    #[arg(long)]
    pub label: Option<Vec<String>>,

    /// metrics associated with the human
    #[arg(long)]
    pub metric: Option<Vec<Metric>>,
}
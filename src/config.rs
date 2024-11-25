use serde::{Deserialize};
use std::fs;
use std::path::Prefix;

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub email: String,
    pub birthday: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub recipients: Vec<Recipient>,
    pub template_path: String,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

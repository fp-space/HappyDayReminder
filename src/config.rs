use serde::Deserialize;
use std::fs;

// 定义 SMTP 配置结构体
#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_email: String,
    pub mode: String,
}

// 定义收件人结构体
#[derive(Debug, Deserialize)]
pub struct Recipient {
    pub name: String,
    pub email: String,
    pub birthday: String,
    reminder_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub recipients: Vec<Recipient>,
    pub template_path: String,
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    println!("加载配置文件: {}", file_path);
    let contents = fs::read_to_string(file_path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

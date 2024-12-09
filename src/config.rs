use serde::{Deserialize, Serialize};
use std::fs;

// 定义 SMTP 配置结构体
#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

// 定义收件人结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Recipient {
    pub name: String,
    pub birthday: String,
    pub calendar_type: Option<String>, // 可选，日历类型，默认为公历"solar"，不然就是农历"lunar"
    reminder_days: Option<i32>,        // 可选，提前提醒天数，默认为0（当天提醒）
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub smtp: SmtpConfig,
    pub recipients: Vec<Recipient>,
}

pub fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // 后面+?表示如果出错，直接返回错误
    let contents = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

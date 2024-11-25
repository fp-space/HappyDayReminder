mod config;
mod mailer;
mod template;
mod email_handler;

use crate::config::load_config;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = load_config("config.yaml")?;

    // 获取当前日期
    let today = chrono::Local::now().naive_local().date(); // Use NaiveDate, not NaiveDateTime

    match config.smtp.mode.as_str() {
        "individual" => email_handler::send_individual_emails(&config, today).await?,
        "summary" => email_handler::send_summary_email(&config, today).await?,
        _ => {
            println!("Unsupported mode: {}", config.smtp.mode);
        }
    }

    Ok(())
}


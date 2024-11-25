mod config;
mod mailer;
mod template;

use crate::config::load_config;
use crate::mailer::send_email;
use crate::template::render_template;
use std::collections::HashMap;
use tokio;
use chrono::{Datelike, NaiveDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = load_config("config.yaml")?;

    // 获取当前日期
    let today = chrono::Local::now().naive_local();

    // 遍历收件人列表
    for recipient in &config.recipients {
        // 解析接收者的生日
        let birthday = NaiveDate::parse_from_str(&recipient.birthday, "%Y-%m-%d")?;

        // 判断今天是否是接收者的生日
        if today.month() == birthday.month() && today.day() == birthday.day() {
            // 创建邮件上下文
            let mut context = HashMap::new();
            context.insert("name".to_string(), recipient.name.clone());

            // 渲染邮件内容
            let content = render_template(&config.template_path, &context)?;

            // 发送邮件
            send_email(&config, recipient, &content).await?;
            println!("Birthday email sent to: {}", recipient.email);
        } else {
            println!("Today is not {}'s birthday", recipient.name);
        }
    }

    Ok(())
}

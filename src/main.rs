mod config;
mod mailer;
mod template;

use crate::config::load_config;
use crate::mailer::send_email;
use crate::template::render_template;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = load_config("config.yaml")?;

    println!("{:?}", config);

    // 遍历收件人列表
    for recipient in &config.recipients {
        // 创建邮件上下文
        let mut context = HashMap::new();
        context.insert("name".to_string(), recipient.name.clone());

        // 渲染邮件内容
        let content = render_template(&config.template_path, &context)?;

        // 发送邮件
        send_email(&config, recipient, &content).await?;
        println!("Email sent to: {}", recipient.email);
    }

    Ok(())
}

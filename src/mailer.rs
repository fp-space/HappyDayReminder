use lettre::{SmtpTransport, Transport};
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::authentication::Credentials;
use crate::config::{Config, Recipient};

pub async fn send_email(config: &Config, recipient: &Recipient, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_host = &config.smtp.host;
    let smtp_port = config.smtp.port;
    let smtp_username = &config.smtp.username;
    let smtp_password = &config.smtp.password;
    let from_email = &config.smtp.from_email;

    // 创建邮件消息
    let email = Message::builder()
        .from(from_email.parse::<Mailbox>()?)
        .to(recipient.email.parse::<Mailbox>()?)
        .subject("Important Update")
        .body(content.to_string())?;

    // SMTP 客户端配置
    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
    let mailer = SmtpTransport::relay(smtp_host)?
        .port(smtp_port)
        .credentials(creds)
        .build();

    // 发送邮件
    mailer.send(&email).await?;
    Ok(())
}

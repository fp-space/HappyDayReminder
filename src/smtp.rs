// 单例模式，stmp注册器，连接stmp后，后续所有的调用都从这里获取stmp实例
use crate::config::SmtpConfig;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub fn send_email(config: &SmtpConfig, content: &str) -> std::result::Result<(), Box<dyn Error>> {
    // 创建邮件
    let email = Message::builder()
        .from(config.username.parse()?)
        .to(config.to_email.parse()?)
        .subject(config.subject.clone())
        .body(String::from(content))?;

    // 配置 SMTP 客户端
    let creds = Credentials::new(config.username.clone(), config.password.clone());

    // 创建 SMTP 传输
    let mailer = SmtpTransport::relay(&config.host)?
        .port(config.port)
        .credentials(creds)
        .build();

    // 发送邮件
    mailer.send(&email)?;

    Ok(())
}

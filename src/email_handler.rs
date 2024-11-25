use crate::config;
use crate::mailer::send_email;
use crate::template::render_template;
use chrono::{Datelike, Duration, NaiveDate};
use serde_json;
use std::collections::HashMap;

// 模式1：单独发送邮件给过生日的人
pub(crate) async fn send_individual_emails(config: &config::Config, today: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    for recipient in &config.recipients {
        let birthday = NaiveDate::parse_from_str(&recipient.birthday, "%Y-%m-%d")?;

        // 检查是否在提醒范围内
        let reminder_date = today + Duration::days(config.smtp.reminder_days);
        if reminder_date.month() == birthday.month() && reminder_date.day() == birthday.day() {
            // 创建邮件上下文
            let mut context = HashMap::new();
            context.insert("name".to_string(), recipient.name.clone());
            context.insert("is_summary".to_string(), "false".to_string()); // Mark as individual email

            // 渲染模板并发送邮件
            let content = render_template(&config.template_path, &context)?;
            send_email(&config, recipient, &content).await?;
            println!("Birthday email sent to: {}", recipient.email);
        }
    }
    Ok(())
}

// 模式2：发送汇总信息给自己
pub(crate) async fn send_summary_email(config: &config::Config, today: NaiveDate) -> Result<(), Box<dyn std::error::Error>> {
    let mut birthdays_within_reminder = Vec::new();

    for recipient in &config.recipients {
        let birthday = NaiveDate::parse_from_str(&recipient.birthday, "%Y-%m-%d")?;

        // 检查是否在提醒范围内
        let reminder_date = today + Duration::days(config.smtp.reminder_days);
        if reminder_date.month() == birthday.month() && reminder_date.day() == birthday.day() {
            birthdays_within_reminder.push(HashMap::from([
                ("name".to_string(), recipient.name.clone()), // No need for `.into()`
                ("email".to_string(), recipient.email.clone()) // No need for `.into()`
            ]));
        }
    }

    // 如果有人过生日，发送汇总邮件到自己的邮箱
    if !birthdays_within_reminder.is_empty() {
        // 创建汇总邮件上下文
        let mut context = HashMap::new();
        context.insert("is_summary".to_string(), "true".to_string()); // Mark as summary email
        context.insert("reminder_days".to_string(), config.smtp.reminder_days.to_string());
        context.insert("birthdays".to_string(), serde_json::to_string(&birthdays_within_reminder)?);

        // 渲染模板
        let summary_content = render_template(&config.template_path, &context)?;

        // 自己作为收件人
        let self_recipient = config::Recipient {
            name: "Self".to_string(),
            email: config.smtp.username.clone(),
            birthday: "".to_string(), // 不需要
        };

        send_email(&config, &self_recipient, &summary_content).await?;
        println!("Summary email sent to yourself.");
    } else {
        println!("No birthdays within {} days.", config.smtp.reminder_days);
    }
    Ok(())
}
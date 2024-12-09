use crate::config::Recipient;
use chrono::*;
use tyme4rs::tyme::solar::SolarDay;

pub struct BirthdayChecker {
    recipients: Vec<Recipient>,
}

impl BirthdayChecker {
    pub fn new(recipients: Vec<Recipient>) -> Self {
        Self { recipients }
    }

    // 获取今天生日的人
    pub fn get_birthday_people(&self) -> Vec<&Recipient> {
        let today = SolarDay::from_ymd(
            Local::now().naive_local().date().year() as isize,
            Local::now().naive_local().date().month() as usize,
            Local::now().naive_local().date().day() as usize,
        );
        println!("今天是 {} {}", today, today.get_lunar_day());
        self.recipients
            .iter()
            .filter(|recipient| is_birthday_today(recipient, &today))
            .collect()
    }
}

// 判断是否生日（公历/农历）
fn is_birthday_today(recipient: &Recipient, today: &SolarDay) -> bool {
    let calendar_type = match &recipient.calendar_type {
        None => "solar".to_string(),
        Some(ct) => match ct.as_str() {
            "lunar" => ct.clone(),
            "solar" => ct.clone(),
            _ => panic!("历法类型错误"),
        },
    };
    if let Some(birthday) = parse_birthday_str(&recipient.birthday) {
        if calendar_type == "lunar" {
            if birthday.get_lunar_day().get_month() == today.get_lunar_day().get_month()
                && birthday.get_lunar_day().get_day() == today.get_lunar_day().get_day()
            {
                return true;
            }
        } else {
            if birthday.get_month() == today.get_month() && birthday.get_day() == today.get_day() {
                return true;
            }
        }
    }
    false
}

/// 将日期字符串转换为 SolarDay
fn parse_birthday_str(date_str: &str) -> Option<SolarDay> {
    let date_parts: Vec<&str> = date_str.split('-').collect();
    if date_parts.len() != 3 {
        return None;
    }

    let year = date_parts[0].parse::<isize>().ok()?;
    let month = date_parts[1].parse::<usize>().ok()?;
    let day = date_parts[2].parse::<usize>().ok()?;

    Some(SolarDay::from_ymd(year, month, day))
}

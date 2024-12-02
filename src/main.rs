use chrono::*;
use config::load_config;
use tyme4rs::tyme::lunar::{self, LunarDay};
use tyme4rs::tyme::solar::SolarDay;

mod config;

fn main() {
    let config = load_config("config.yml").unwrap();
    // 获取当前日期
    let today = Local::now().naive_local().date();
    let mut day = SolarDay::from_ymd(
        today.year().try_into().unwrap(),
        today.month().try_into().unwrap(),
        today.day().try_into().unwrap(),
    );
    println!("今天是 公历 {} 农历 {}", day, day.get_lunar_day());
}

use config::load_config;
mod checker;
use checker::BirthdayChecker;

mod config;

fn main() {
    // 加载配置文件
    let config = load_config("config.yml").unwrap();

    // 遍历收件人，获取今天生日的人
    let checker = BirthdayChecker::new(config.recipients);
    let birthday_people = checker.get_birthday_people();
    println!("今天生日的人：{:?}", birthday_people);


    // 整合到模板中

    // 发送邮件
}

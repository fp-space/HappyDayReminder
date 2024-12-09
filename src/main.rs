mod checker;
mod config;
use checker::BirthdayChecker;
use config::load_config;
use handlebars::Handlebars;
use serde_json::json;
use chrono::Local;


fn main() {
    // 加载配置文件
    let config = load_config("config.yml").unwrap();

    // 遍历收件人，获取今天生日的人
    let checker = BirthdayChecker::new(config.recipients);
    let birthday_people = checker.get_birthday_people();
    println!("今天生日的人：{:?}", birthday_people);

    // 整合到模板中
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("birthday", "templates/birthday.hbs")
        .unwrap();

    let data = json!({
        "date": Local::now().format("%Y-%m-%d").to_string(),
        "birthday_people": birthday_people
    });

    let content = handlebars.render("birthday", &data).unwrap();
    println!("生成的邮件内容:\n{}", content);

    // 发送邮件
}

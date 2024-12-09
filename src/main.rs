mod checker;
mod config;
use checker::BirthdayChecker;
use config::load_config;
mod template;



fn main() {
    // 加载配置文件
    let config = load_config("config.yml").unwrap();

    // 遍历收件人，获取今天生日的人
    let checker = BirthdayChecker::new(config.recipients);
    let birthday_people = checker.get_birthday_people();

    // 通过模板渲染邮件内容
    let content = template::render_email_content("birthday_template", birthday_people);
    println!("\n生成邮件内容:\n{}", content);

    // 发送邮件
}

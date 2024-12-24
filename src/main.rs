mod checker;
mod config;
mod smtp;
mod template;

use checker::BirthdayChecker;
use config::load_config;
use smtp::send_email;
use template::render_email_content;

fn main() {
    // 加载配置文件
    let config = load_config("config.yml").expect("无法加载配置文件");

    // 遍历收件人，获取今天生日的人
    let checker = BirthdayChecker::new(config.recipients);
    let birthday_people = checker.get_birthday_people();
    if birthday_people.is_empty() {
        println!("今天没有人过生日！");
        return;
    }

    // 通过模板渲染邮件内容
    let content = render_email_content("birthday_template", birthday_people);
    println!("\n生成邮件内容:\n{}", content);

    // 发送邮件
    if let Err(e) = send_email(&config.smtp, &content) {
        eprintln!("发送邮件失败: {}", e);
    } else {
        println!("邮件发送成功！");
    }
}

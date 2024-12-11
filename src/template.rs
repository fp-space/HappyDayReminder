// 通过模板渲染生成html

use crate::config::Recipient;
use chrono::Local;
use handlebars::Handlebars;
use serde_json::json;
use std::fs;

pub fn render_email_content(template_name: &str, birthday_people: Vec<&Recipient>) -> String {
    let mut handlebars = Handlebars::new();

      // 读取 tailwind css 文件
    let tailwind_css = fs::read_to_string("html/preview/css/output.css")
        .expect("Failed to read Tailwind CSS file");

    // 处理数据
    let data = json!({
        "tailwind": tailwind_css,
        "birthday_people": birthday_people,
        "date": Local::now().format("%Y-%m-%d").to_string(),
    });

    // 注册模板
    handlebars
        .register_template_file(
            template_name,
            format!("html/templates/{}.hbs", template_name),
        )
        .expect("Failed to register template");

    // 渲染模板
    handlebars
        .render(template_name, &data)
        .expect("Failed to render template")
}

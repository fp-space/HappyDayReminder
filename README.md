# 🎂 HappyDayReminder

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![Rust Version](https://img.shields.io/badge/rust--version-1.70%2B-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

</div>

一个简单实用的生日提醒工具,可以自动发送生日提醒邮件,支持公历和农历生日,可自定义提醒模板

## ✨ 特性

- [X] 支持公历和农历生日提醒
- [X] SMTP邮件发送
- [X] 自定义 Handlebars 邮件模板
- [X] 支持在配置文件批量管理多个生日提醒
- [X] 可预览渲染后的电子邮件

## 🛠 开发计划

- [ ] 错误处理
- [ ] 配置提前提醒天数
- [ ] 月度生日汇总报告
- [ ] 支持更多通知渠道...

## 🚀 快速开始

将`config.example.yml`修改为`config.yml`后，在其中修改STMP相关信息

要添加好友的生日信息，只需在`recipients`字段中添加相应条目即可

邮件模板文件位于`./html/templates`中，如果需要修改模板，请查看[预览电子邮件](./html/preview/README.md)

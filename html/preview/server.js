const express = require('express');
const Handlebars = require('handlebars');
const fs = require('fs');
const path = require('path');

const app = express();
const port = 3000;

// 读取 tailwind css 文件
const tailwindCss = fs.readFileSync(
    path.join(__dirname, './css/output.css'),
    'utf-8'
);

// 读取模板文件
const templateFile = fs.readFileSync('../templates/birthday_template.hbs', 'utf-8');
const template = Handlebars.compile(templateFile);

// 测试数据
const testData = {
    tailwind: tailwindCss,
    date: '2024年3月21日',
    birthday_people: [
        { name: "用户1", birthday: "2023-12-23", calendar_type: "lunar", reminder_days: null },
        { name: "用户2", birthday: "2024-12-11", calendar_type: null, reminder_days: null }
    ]
};

app.get('/', (req, res) => {
    // 渲染模板
    const html = template(testData);
    res.send(html);
});

app.listen(port, () => {
    console.log(`预览服务器运行在 http://localhost:${port}`);
});
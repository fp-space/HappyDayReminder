# 邮件模板预览

> 如果无需预览，直接查看`templates`文件夹内的模板即可

进入 `html` 目录安装依赖:

> 以下所有命令都在 `html` 目录下执行

```shell
cd templates
npm install
```

配置tailwindcss:

```shell
npx tailwindcss -i ./css/input.css -o ./css/output.css --watch
```

启动预览服务:

```shell
npx nodemon server.js
```

访问 [http://localhost:3000](http://localhost:3000) 查看邮件渲染效果


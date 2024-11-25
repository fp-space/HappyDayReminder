# 使用官方 Rust 镜像作为构建阶段
FROM rust:1.72 as builder

# 设置工作目录
WORKDIR /app

# 复制 Cargo.toml 和 Cargo.lock 文件
COPY Cargo.toml Cargo.lock ./

# 创建一个临时的 src 目录以利用构建缓存
RUN mkdir src && echo "fn main() {}" > src/main.rs

# 构建依赖，缓存层
RUN cargo build --release
RUN rm -rf src

# 复制项目所有文件到工作目录
COPY . .

# 编译项目
RUN cargo build --release

# 使用更小的运行时镜像
FROM debian:buster-slim

# 安装运行时依赖库（如 OpenSSL）
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建阶段复制编译后的可执行文件
COPY --from=builder /app/target/release/HappyDayReminder /usr/local/bin/HappyDayReminder

# 复制运行时需要的文件，例如配置文件和模板
COPY config.yaml .
COPY email_template.hbs .

# 设定环境变量，默认配置文件路径
ENV CONFIG_PATH=/app/config.yaml

# 设定容器启动时运行的命令
CMD ["HappyDayReminder"]

# 使用基于 Ubuntu 的 Rust 官方镜像，减少基础镜像的构建时间
FROM rust:1.82-bullseye

# 更新系统并安装必要的依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v23.4/protoc-23.4-linux-x86_64.zip \
    && unzip protoc-23.4-linux-x86_64.zip -d /usr/local \
    && rm protoc-23.4-linux-x86_64.zip

# 设置工作目录
WORKDIR /app

# 复制 Cargo.toml 和 Cargo.lock 文件
COPY Cargo.toml Cargo.lock ./

# 复制项目源代码 bff，comment，interactive，note，protos，user, im 目录
COPY bff ./bff
COPY comment ./comment
COPY interactive ./interactive
COPY note ./note
COPY protos ./protos
COPY user ./user
COPY im ./im

# 编译项目
RUN cargo build --release

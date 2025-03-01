# 使用官方 Rust 镜像作为构建基础镜像
FROM rust:1.85.0-slim AS builder

# 安装必要的依赖（可选）
RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /workspace

# 拷贝工作空间根目录的 Cargo.toml 和 Cargo.lock
COPY Cargo.toml Cargo.lock ./

# 清理临时文件
RUN rm -rf /tmp/*

# 拷贝源代码到 builder 镜像
COPY . .
#RUN export RUSTUP_DIST_SERVER="https://rsproxy.cn"
#RUN export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
# 构建项目
RUN cargo build --release --locked

FROM ubuntu:24.04
# 安装运行时依赖
RUN apt-get update \
    && rm -rf /var/lib/apt/lists/* \

# 暴露服务端口
EXPOSE 8080

# 设置工作目录
WORKDIR /app

# 从 builder 镜像中拷贝构建好的二进制文件
COPY --from=builder /workspace/target/release/web-tpl /app/web-api-bin

# 启动应用
CMD ["./web-api-bin"]

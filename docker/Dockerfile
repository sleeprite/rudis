# ============================================================
# Stage 1: Install cargo-chef (依赖分析工具)
# ============================================================
FROM rust:1.95.0-alpine AS chef

RUN apk add --no-cache musl-dev
RUN cargo install cargo-chef --locked

WORKDIR /rudis

# ============================================================
# Stage 2: 生成 recipe.json（依赖配方文件）
# ============================================================
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY benches ./benches

RUN cargo chef prepare --recipe-path recipe.json

# ============================================================
# Stage 3: 使用 recipe.json 构建依赖并编译项目
# ============================================================
FROM chef AS builder

COPY --from=planner /rudis/recipe.json recipe.json

# 先构建依赖（这一步会被 Docker 缓存，只有 Cargo.toml/lock 变化时才重跑）
RUN cargo chef cook --release \
    --target x86_64-unknown-linux-musl \
    --recipe-path recipe.json

# 复制完整源码并构建
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release \
    --target x86_64-unknown-linux-musl

# ============================================================
# Stage 4: 极简运行环境
# ============================================================
FROM alpine:3.22.4

WORKDIR /rudis

COPY --from=builder \
    /rudis/target/x86_64-unknown-linux-musl/release/rudis-server .

COPY static ./static

EXPOSE 6379
EXPOSE 7379

VOLUME ["/rudis"]

ENTRYPOINT ["./rudis-server"]

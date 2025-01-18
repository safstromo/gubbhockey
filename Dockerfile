# Get started with a build env with Rust nightly
# FROM rustlang/rust:nightly-alpine as builder
#
# RUN apk update && \
#     apk add --no-cache bash curl npm libc-dev binaryen openssl-dev clang
#
# RUN npm install -g sass
#
# RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
#
# # Add the WASM target
# RUN rustup target add wasm32-unknown-unknown
#
# # Clear npm cache and remove package-lock.json and node_modules
# RUN npm cache clean --force && \
#     rm -rf package-lock.json node_modules
#
# WORKDIR /work
# COPY . .
#
# RUN cargo update -p wasm-bindgen --precise 0.2.100
# RUN cargo install sqlx-cli
# RUN cargo sqlx prepare
# RUN cargo leptos build --release -vv
#
# FROM rustlang/rust:nightly-alpine as runner
#
# WORKDIR /app
#
# COPY --from=builder /work/target/release/gubbhockey /app/
# COPY --from=builder /work/target/site /app/site
# COPY --from=builder /work/Cargo.toml /app/
#
# EXPOSE 3000
# ENV RUST_LOG="info"
# ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
# ENV LEPTOS_SITE_ROOT=./site
#
# CMD ["/app/gubbhockey"]

FROM ghcr.io/ekshore/cargo-leptos-runner-nightly:latest AS builder

WORKDIR /build

COPY . .

RUN cargo update -p wasm-bindgen --precise 0.2.100
RUN cargo install sqlx-cli
RUN cargo sqlx prepare --check

RUN cargo leptos build --release

FROM ubuntu:plucky AS runner

WORKDIR /app

COPY --from=builder /build/target/release/gubbhockey /app/gubbhockey
COPY --from=builder /build/target/site /app/site

RUN useradd -ms /bin/bash app
USER app

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
ENV DATABASE_URL="postgres://develop:develop@localhost:5432/gubbhockey"
ENV SQLX_OFFLINE=true
 
EXPOSE 3000

CMD [ "/app/gubbhockey" ]

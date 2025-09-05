FROM lukemathwalker/cargo-chef:latest-rust-1.89.0-trixie AS chef
WORKDIR /app

ENV MALLOC_ARENA_MAX=2

RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev \
    && rustup target add x86_64-unknown-linux-musl

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./core ./core
COPY ./presentation ./presentation
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/presentation /usr/local/bin/app

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/app"]
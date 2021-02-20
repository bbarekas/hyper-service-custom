FROM rust:1.50 as builder

RUN USER=root cargo new --bin hyper-service-custom
WORKDIR ./hyper-service-custom

COPY ./Cargo.toml   ./Cargo.toml
COPY ./src/         ./src/

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /hyper-service-custom/target/release/hyper-service-custom /hyper-service-custom
USER root

ENV PORT=3001
ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

CMD ["/hyper-service-custom"]
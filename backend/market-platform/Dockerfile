FROM rust:latest
LABEL authors="simon"

WORKDIR /usr/src/market-platform
COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8001

CMD sleep 30 ; diesel migration run ; /usr/src/market-platform/target/release/market-platform

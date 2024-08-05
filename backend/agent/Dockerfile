FROM rust:latest
LABEL authors="simon"

WORKDIR /usr/src/agent
COPY . .

RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0

CMD sleep 30 ; /usr/src/agent/target/release/agent

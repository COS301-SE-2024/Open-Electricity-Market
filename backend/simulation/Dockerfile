FROM rust:1.70
LABEL authors="simon"

WORKDIR /usr/src/simulation
COPY . .

RUN cargo install --path .

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD ["simulation"]
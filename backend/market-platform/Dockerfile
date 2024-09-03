FROM rust:latest
LABEL authors="simon"

RUN wget https://github.com/mozilla/sccache/releases/download/v0.8.1/sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz  \
    && mv sccache-v0.8.1-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache


WORKDIR /usr/src/market-platform
COPY . .

RUN --mount=type=cache,mode=0777,target=/home/root/.cache/sccache \ 
	cargo install diesel_cli --no-default-features --features postgres && sccache --show-stats

RUN --mount=type=cache,mode=0777,target=/home/root/.cache/sccache \ 
	cargo build --release && sccache --show-stats

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8001

CMD diesel migration run ; /usr/src/market-platform/target/release/market-platform

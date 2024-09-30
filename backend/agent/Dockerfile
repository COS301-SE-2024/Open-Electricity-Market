FROM rust:latest
LABEL authors="simon"

RUN wget https://github.com/mozilla/sccache/releases/download/v0.8.1/sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.8.1-x86_64-unknown-linux-musl.tar.gz  \
    && mv sccache-v0.8.1-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache

ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_DIR=/home/root/.cache/sccache


WORKDIR /usr/src/agent
COPY . .

RUN --mount=type=cache,mode=0777,target=/home/root/.cache/sccache \  
	cargo build --release && sccache --show-stats

ENV ROCKET_ADDRESS=0.0.0.0

CMD curl -H 'Content-Type: application/json' -X POST http://simulation:8000/start ;/usr/src/agent/target/release/agent

FROM rust:trixie AS builder

WORKDIR /app
COPY . .

RUN apt-get update -y
RUN apt-get install -y \
  build-essential \
  pkg-config \
  libharfbuzz-dev \
  libgraphite2-dev \
  libicu-dev

RUN cargo build --release
RUN ldd target/release/aleksandr-vinokurov-cv

FROM debian:trixie-slim

COPY --from=builder /app/target/release/aleksandr-vinokurov-cv /usr/local/bin/aleksandr-vinokurov-cv

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
    ca-certificates \
    libicu76 \
    libfontconfig1 \
    libfreetype6 \
    libgraphite2-3 \
    libpng16-16 \
 && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["aleksandr-vinokurov-cv"]
WORKDIR /out

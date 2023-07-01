FROM rust:1.70.0 as builder

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY src ./src
COPY migrations ./migrations
COPY .env .env
RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y \
    libssl-dev \
    sqlite3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/jourst /usr/local/bin/jourst

WORKDIR /usr/local/bin

ENV DATABASE_URL=jourst_db.db

CMD ["jourst"]
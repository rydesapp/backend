
FROM rust:1.47.0 AS builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# We need to touch our real main.rs file or
# else docker will use the cached one.
COPY src src
RUN touch src/main.rs
RUN cargo build --release
RUN cargo install --version=0.1.0-beta.1 sqlx-cli --no-default-features --features postgres
#
# Size optimization
#RUN strip /app/target/release/rydes-api/

FROM ubuntu
RUN apt-get update \
    && apt-get install libssl1.1 libssl-dev

WORKDIR /app
COPY --from=builder /app/target/release/rydes-api .
COPY --from=builder /usr/local/cargo/bin/sqlx .
COPY migrations migrations
EXPOSE 3000
ENV RUST_LOG=main
ENTRYPOINT ["./rydes-api"]


FROM rust:1.47.0 AS builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# We need to touch our real main.rs file or
# else docker will use the cached one.
COPY src src
COPY sqlx-data.json sqlx-data.json
COPY migrations migrations
RUN touch src/main.rs
RUN cargo build --release

# Size optimization
#RUN strip /app/target/release/rydes-api/

FROM ubuntu
RUN apt-get update \
    && apt-get install libssl1.1 libssl-dev
WORKDIR /app
COPY --from=builder /app/target/release/rydes-api .
ENTRYPOINT ["./rydes-api"]

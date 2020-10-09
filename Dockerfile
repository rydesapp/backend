FROM yasuyuky/rust-ssl-static@sha256:3df2c8949e910452ee09a5bcb121fada9790251f4208c6fd97bb09d20542f188 as build

COPY ./ ./

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build --release

RUN mkdir -p /build-out

RUN cp target/release/rydes-api /build-out/

RUN ls /build-out/

FROM scratch

COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

COPY --from=build /build-out/rydes-api /

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

CMD ["/rydes-api"]

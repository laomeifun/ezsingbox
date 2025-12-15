# syntax=docker/dockerfile:1

# Build ezsingbox as a static-ish musl binary so it can run on top of the official sing-box image.
FROM rust:1.85-alpine AS builder

WORKDIR /work

RUN apk add --no-cache build-base musl-dev pkgconfig
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

# Final image: official sing-box image
FROM ghcr.io/sagernet/sing-box:latest

COPY --from=builder /work/target/x86_64-unknown-linux-musl/release/ezsingbox /usr/local/bin/ezsingbox

ENV EZ_CONFIG_PATH=/etc/sing-box/config.json
ENV EZ_LOG_LEVEL=info
# If the base image PATH does not contain sing-box, set SING_BOX_BIN to the absolute path.
ENV SING_BOX_BIN=sing-box

ENTRYPOINT ["/usr/local/bin/ezsingbox"]
CMD ["run"]

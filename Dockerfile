# syntax=docker/dockerfile:1

# Build ezsingbox as a static-ish musl binary so it can run on top of the official sing-box image.
FROM rust:1.85-alpine AS builder

WORKDIR /work

RUN apk add --no-cache build-base musl-dev pkgconfig

# Buildx will set TARGETARCH for multi-arch builds.
ARG TARGETARCH
RUN case "$TARGETARCH" in \
			amd64) echo x86_64-unknown-linux-musl > /tmp/rust_target ;; \
			arm64) echo aarch64-unknown-linux-musl > /tmp/rust_target ;; \
			*) echo "Unsupported TARGETARCH=$TARGETARCH" >&2; exit 1 ;; \
		esac
RUN rustup target add "$(cat /tmp/rust_target)"

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release --target "$(cat /tmp/rust_target)"

RUN mkdir -p /out \
	&& cp "/work/target/$(cat /tmp/rust_target)/release/ezsingbox" /out/ezsingbox

# Final image: official sing-box image
FROM ghcr.io/sagernet/sing-box:latest

COPY --from=builder /out/ezsingbox /usr/local/bin/ezsingbox

ENV EZ_CONFIG_PATH=/etc/sing-box/config.json
ENV EZ_LOG_LEVEL=info
# If the base image PATH does not contain sing-box, set SING_BOX_BIN to the absolute path.
ENV SING_BOX_BIN=sing-box

ENTRYPOINT ["/usr/local/bin/ezsingbox"]
CMD ["run"]

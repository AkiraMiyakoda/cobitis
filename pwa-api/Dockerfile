# Copyright © 2024 Akira Miyakoda
#
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

# Build server

FROM rust:1-bookworm AS builder

WORKDIR /tmp/build

COPY ./Cargo.* .
RUN mkdir ./src && \
    echo "fn main() {}" > ./src/main.rs && \
    cargo build --release

COPY ./src ./src
RUN touch ./src/main.rs && \
    cargo build --release

# Copy server to final image

FROM debian:bookworm-slim

COPY --from=builder --chmod=755 ./tmp/build/target/release/pwa-api /usr/bin

ENTRYPOINT [ "pwa-api" ]

FROM rust:1.68.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN rustup override set nightly; \
    cargo install --path .

FROM ubuntu:20.04
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
ENV AXUM_PORT 8080
CMD myapp
# from https://github.com/emk/rust-musl-builder
FROM ekidd/rust-musl-builder:1.50.0 AS builder
ADD --chown=rust:rust . ./
RUN cargo build --release

# install ca-certificates
FROM alpine:latest as certs
RUN apk update && apk add ca-certificates

# final. application layer
FROM busybox:musl
COPY --from=certs /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/wasmaas ./
CMD ["./wasmaas"]

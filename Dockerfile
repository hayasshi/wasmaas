# from https://github.com/emk/rust-musl-builder
FROM ekidd/rust-musl-builder:1.51.0 AS builder
ADD --chown=rust:rust . ./
ADD --chown=rust:rust double.wasm ./
ADD --chown=rust:rust greet.wasm ./
RUN cargo build --release

# install ca-certificates
FROM alpine:latest as certs
RUN apk update && apk add ca-certificates

# final. application layer
FROM busybox:musl
COPY --from=certs /etc/ssl/certs /etc/ssl/certs
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/wasmaas ./
COPY --from=builder /home/rust/src/double.wasm ./
COPY --from=builder /home/rust/src/greet.wasm ./
CMD ["./wasmaas"]

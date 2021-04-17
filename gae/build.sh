#!/bin/bash

set -e

SCRIPT_DIR=$(cd $(dirname $0); pwd)
PROJECT_DIR=$(cd $SCRIPT_DIR/../; pwd)
CACHE_DIR="$PROJECT_DIR/.build-cache/gae"

BUILD_IMAGE="ekidd/rust-musl-builder:1.50.0"
BUILD_CMD="cargo build --release"

docker run --rm -it \
  -v $PROJECT_DIR:/home/rust/src \
  -v $CACHE_DIR/cargo-git:/home/rust/.cargo/git \
  -v $CACHE_DIR/cargo-registry:/home/rust/.cargo/registry \
  -v $CACHE_DIR/target:/home/rust/src/target \
  $BUILD_IMAGE $BUILD_CMD

cp "$CACHE_DIR/target/x86_64-unknown-linux-musl/release/wasmaas" $SCRIPT_DIR

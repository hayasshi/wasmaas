# wasmaas

Proof of concept of `WASM as a Service`.

## Features

- upload wasm and create space(url path) for it
- pass through request to wasm

## local build & run

```
docker compose up --build
```

## Release to GAE

```
bash gae/build.sh  # unnecessary if release binary is already builded
gcloud app deploy gae/app.yaml
```

## Build wasm example

#### Needed additional tools

```
rustup target add wasm32-wasi
```

If you want to use cargo conveniently for wasm, you can install `cargo-wasi` and [wasmtime](https://github.com/bytecodealliance/wasmtime#installation)

```
curl https://wasmtime.dev/install.sh -sSf | bash
cargo install cargo-wasi
```

#### Build WASM(WASI)

```
cargo build --target wasm32-wasi --release -p greet

# If you have cargo-wasi installed, you can also do the following
cargo wasi build --release -p greet
```

## todo

- Impl file upload on actix-web
- Impl file put to CloudStrage
- Impl data put to CloudDatastore
- Impl exec wasm on wasmtime
- Impl features

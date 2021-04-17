# wasmaas

Proof of concept of `WASM as a Service`.

## features

- upload wasm and create space(url path) for it
- pass through request to wasm

## local build & run

```
docker compose up --build
```

## release to GAE

```
bash gae/build.sh  # unnecessary if release binary is already builded
gcloud app deploy gae/app.yaml
```

## todo

- Impl file upload on actix-web
- Impl file put to CloudStrage
- Impl data put to CloudDatastore
- Impl exec wasm on wasmtime
- Impl features

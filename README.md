# 环境安装命令

## csv

```bash
cargo add clap --features derive

cargo add duckdb --features bundled

cargo add csv

cargo add serde --features derive

cargo add anyhow

cargo add serde-json

cargo add serde-yaml

cargo install tokei
```

## rand

```bash
cargo add rand

cargo add zxcvbn

```

## base64

```bash
cargo add base64

```

## Testsign

```bash
cargo add blake3

cargo add ed25519_dalek

cargo add ed25519_dalek --features rand_core
```

## Http

```bash
cargo add tokio --features rt --features rt-multi-thread --features macros --features net --features fs

cargo add tracing-subscriber --features env-filter

cargo add axum --features http2 --features query --features tracing

cargo add tower-http --features compression-full --features cors --features trace --features fs
```

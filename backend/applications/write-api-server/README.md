# write-api-server

Write API Server です.

AWS Lambda 上で動きます.
少し書き換えれば EC2 でも動きます.

実体は [command-infrastructure](../../command/infrastructure/) にあります.

## 実行確認

Docker が起動されている事が前提です. 起動方法は [こちら](../../README.md).

[main.rs](./src/main.rs), [controllers.rs](../../command/infrastructure/src/controllers.rs) をローカル実行用にコメントアウト, コメントアウト解除し, 下記のコマンドを実行.

```bash
# /backend
cargo make run-write-server
```

## ビルド

```bash
# /backend
cargo make build-write-server
```

[target/lambda/write-api-server](../../target/lambda/write-api-server) に `bootstrap` が出力されるので, zip 化して Lambda にアップロードします.

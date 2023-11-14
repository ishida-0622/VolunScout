# write-api-server

Write API Server です

AWS Lambda 上で動きます. 少し書き換えれば EC2 でも動きます

今のところはローカルだと動きません

実体は [command-infrastructure](../../command/infrastructure/) にあります.

## ビルド

Docker が起動されている事が前提です. 起動方法は [こちら](../../README.md)

```bash
cargo lambda build --release -p write-api-server --bin write-api-server
```

[target/lambda/write-api-server](../../target/lambda/write-api-server) に `bootstrap` が出力されるので, zip 化して Lambda にアップロードします

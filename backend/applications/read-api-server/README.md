# read-api-server

Read API Server です.

EC2 上で動かすことを想定しています. ローカルでも動きます.

実体は [query-infrastructure](../../query/infrastructure) にあります.

## 実行確認

Docker 上で MySQL が起動していることが前提です. 起動方法は [こちら](../../README.md).

```bash
# /backend
cargo make run-read-server
```

[localhost:8080](http://localhost:8080) が起動します

[/graphql](http://localhost:8080/graphql) にアクセスすると GraphQL IDE が開きます

## ビルド

```bash
# /backend
cargo make build-read-server
```

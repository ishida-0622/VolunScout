# read-api-server

Read API Server です

EC2 上で動かすことを想定しています. ローカルでも動きます

実体は [query-infrastructure](../../query/infrastructure) にあります

## 実行確認

サーバー起動

```bash
cargo run -p read-api-server --bin read-api-server
```

[localhost:8080](http://localhost:8080) が起動します. [/graphql](http://localhost:8080/graphql)にアクセスすると GraphQL IDE が開きます

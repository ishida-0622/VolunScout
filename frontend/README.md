# frontend

フロントエンドです. Next.js です.

## 環境構築

依存関係のインストール.

```bash
# /frontend
npm install
```

開発サーバーを起動.

```bash
# /frontend
npm run dev
```

[http://localhost:3000](http://localhost:3000) が起動します.

## 各種コマンドなど

### DB 起動

[こちら](../backend/README.md)を参照.

### GraphQL スキーマからクエリのコードを生成

```bash
# /frontend
npm run generate-graphql # 一度だけ
# or
npm run generate-graphql-w # 変更を監視
```

### Open API スキーマからコマンドのコードを生成

```bash
# /frontend
npm run generate-openapi
# or
cargo run generate-openapi # Docker image が必要
```

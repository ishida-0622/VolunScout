## Docker 関連

各種ツールのセットアップは [こちら](./tools/README.md)

### MySQL の起動

```bash
cd backend/tools
make db-up
```

### テストデータの挿入

```bash
make db-init-insert
```

### MySQL の終了

```bash
make db-down
```

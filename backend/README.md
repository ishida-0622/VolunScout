# backend

バックエンドです. Rust です.

## フォルダ構成

弊社（予定）のインターンで使用したものを参考にしています.

クリーンアーキテクチャを踏襲しているらしいです.

- command: コマンド側
  - domain: ドメイン層
  - command-repository: コマンド側のインターフェース定義
  - command-infrastructure: コマンド側のインターフェース実装
- query: クエリ側
  - query-repository: クエリ側のインターフェース定義
  - query-infrastructure: クエリ側のインターフェース実装. GraphQL の記述がメイン

## 環境構築

### Docker 関連

各種ツールのセットアップは [こちら](./tools/README.md).

#### MySQL の起動

```bash
# /backend/tools
make db-up
```

`Error response from daemon: Ports are not available: exposing port TCP 0.0.0.0:3306 -> 0.0.0.0:0: listen tcp 0.0.0.0:3306: bind: Only one usage of each socket address (protocol/network address/port) is normally permitted.`

上記のエラーが出たら, 管理者権限のコマンドプロンプトで以下を実行してください.

```bash
net stop mysql81
```

#### テストデータの挿入

```bash
# /backend/tools
make db-init-insert
```

#### MySQL の終了

```bash
# /backend/tools
make db-down
```

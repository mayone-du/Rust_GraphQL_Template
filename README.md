# Rust で GraphQL サーバーを構築するためのテンプレート（予定）

## 使用技術

- Rust
- juniper
- PostgreSQL

## セットアップ

.env を作成
build は Docker コンテナ内で行うのではなく、自分の環境で build しないと遅くなる。

開発環境に無理に Docker を使わなくてもいいかもなので、要検討。

### diesel の使用

.env に DB の URL を記載した状態で、diesel setup を実行すると migrations フォルダが作成される。
その後、diesel migration run で DB がセットアップされる？
自分でテーブル定義をするときは、diesel migration generate <その回の名前>を実行し、そこで作成された up.sql に SQL を書く。
その後、diesel migration run でテーブルが定義され、schema.rs に table!マクロが記載される。

## TODO

- DB 接続
- ...

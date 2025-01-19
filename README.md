# 『バックエンドエンジニアを目指す人のためのRust』 第10章 TODO Webアプリ

# 環境構築
## プロジェクトの作成
```
cargo new todo-web
```

## 各外部クレートの導入
```
cargo add actix-web
cargo add askama
cargo add askama_actix
cargo add sqlx --features sqlite,runtime-tokio migration
cargo add serde --features derive
cargo add dotenv
```

## 改善
* [x] hello機能を削除
* [x] SQLiteはインメモリーではなく、ファイルに保存
* [x] SQLiteのファイル名は、コード直書きではなく、.envから取得
* [ ] DoneはDELETEではなく、UPDATE

#　使い方
## 起動
```
cargo run
```

## Hello　Rust
```
http://127.0.0.1:8080/hello/rust
```

## TODO
```
http://127.0.0.1:8080/
```

以上
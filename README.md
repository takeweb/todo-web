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
* [x] main以外のロジックをlib.rsへ分離
* [x] DoneはDELETEではなく、UPDATE
* [x] ステータスを「未着手(0)」、「仕掛かり中(1)」、「完了(9)」の管理に変更
* [x] UIは、左から横並びで「未着手(Unstarted)」、「仕掛かり中(In progress)」、「完了(Done)」が並ぶ
* [x] ステータスを「未着手(Unstarted)」、「仕掛かり中(In progress)」、「完了(Done)」の管理に変更
* [ ] ステータスが「未着手(Unstarted)」の場合は、タスク内容の編集可能
* [x] ステータスが「未着手(Unstarted)」で追加されたタイミングで「追加日」を現在日時で更新
* [x] ステータスに「仕掛かり中(In progress)」を追加
* [x] ステータスが「仕掛かり中(In progress)」になったタイミングで、「着手日」を現在日時で更新
* [ ] ステータスを「完了(Done)」から「仕掛かり中(In progress)」に戻す
* [x] ステータスが「完了(Done)」になったタイミングで、「完了日」を現在日時で更新
* [ ] UIにBootstrap 5の導入
* [ ] DATETIME型はUTCで保存される為、表示する際は＋9時間する必要がある
* [ ] 一覧画面の実装
* [ ] UIでタスク追加時に期限を設定できるようにする
* [ ] [ctrl + c]で終了すると、*.db-shmや*.db-walが残るのをどうにかしたい
* [ ] UIでタスクをドラッグで範囲に移動することで、ステータス変更を可能とする
* [ ] 親タスク、子タスクの管理を可能とする
* [ ] タスク全体をプロジェクトとして、独立管理可能とする
* [ ] プロジェクトに名前を保持できるようにする
* [ ] プロジェクト毎に参照可能ユーザを設定可能とする
* [ ] タスク毎に期限を管理可能とする

#　使い方
## 起動
```
cargo run
```

## TODO
```
http://127.0.0.1:8080
or
http://localhost:8080
```

以上
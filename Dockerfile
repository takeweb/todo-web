# ---------------------------------------------------------------
# ビルドステージ
# Rustのバイナリをビルドするために必要なツールと環境を準備します
# ---------------------------------------------------------------

# Stage 1: cargo-chefとその他のビルドツールのインストール
# このステージでは、後続のステージで必要となるcargo-chefをインストールします。
FROM rust:latest AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked
# 必要に応じて、システムレベルの依存関係（例: openssl-dev, pkg-config）をインストールします。
# これらはRustクレートがCライブラリに依存する場合に必要になることがあります。
# RUN apt-get update && apt-get install -y libssl-dev pkg-config --no-install-recommends && rm -rf /var/lib/apt/lists/*

# Stage 2: 依存関係の計画ステージ (Planner Stage)
# このステージでは、Cargo.tomlとCargo.lockを基に依存関係の「レシピ」を生成します。
# このレシピは、依存関係に変更があった場合にのみ再生成されます。
FROM chef AS planner
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# .cargo/config.tomlを作成し、sparse registryを有効にします。
# これにより、crates.ioのインデックス更新が高速化され、ビルド時間の短縮に貢献します。
RUN mkdir -p .cargo && \
    echo '[registries.crates-io]' >> .cargo/config.toml && \
    echo 'protocol = "sparse"' >> .cargo/config.toml
# ダミーのsrc/main.rsを作成して、cargo chefが有効なターゲットを見つけられるようにします。
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
# cargo-chefを使って依存関係のレシピを生成します。
RUN cargo chef prepare --recipe-path recipe.json

# Stage 3: 依存関係のビルドステージ (Builder Stage)
# このステージでは、生成されたレシピに基づいて依存関係をコンパイルします。
# `url`クレートのような時間がかかる依存関係のコンパイルはここで実行され、Dockerのキャッシュに保存されます。
# 依存関係に変更がない限り、このステージはキャッシュから再利用されるため、ビルドが大幅に高速化されます。
FROM chef AS builder
WORKDIR /app
# 生成されたレシピをコピーします。
COPY --from=planner /app/recipe.json recipe.json
# sparse registryの設定をこのステージにも適用します。
RUN mkdir -p .cargo && \
    echo '[registries.crates-io]' >> .cargo/config.toml && \
    echo 'protocol = "sparse"' >> .cargo/config.toml
# cargo-chefを使って依存関係をコンパイルします。
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 4: アプリケーションコードのビルドステージ (App Builder Stage)
# このステージでは、依存関係がすでにコンパイルされているため、アプリケーション自身のコードのみをビルドします。
# これにより、ソースコードの変更があった場合でも、ビルド時間が劇的に短縮されます。
FROM builder AS app_builder
WORKDIR /app
# プロジェクトのソースコードを全てコピーします。
COPY . .
# SQLx CLIをインストール（マイグレーションのため）。
# SQLx CLIはビルド時にのみ必要なので、このステージでインストールします。
RUN cargo install sqlx-cli --no-default-features --features rustls,sqlite

# データベースファイルの初期化とマイグレーションの実行
# アプリケーションが参照するパスに合わせて設定します。
ENV DATABASE_URL="sqlite://db/database.db"

# データベースディレクトリが存在しない場合に作成します。
RUN mkdir -p db

# マイグレーションファイルが存在する`migrations`ディレクトリをコピーします。
# `COPY . .`で既にコピーされますが、明示的に記述することで意図を明確にします。
COPY migrations ./migrations

# マイグレーションを実行します。
# ここでdb/database.dbが作成され、スキーマが適用されます。
# マイグレーションが成功したことを確認するために、ファイルが存在するかチェックします。
RUN sqlx migrate run && ls -l db/database.db

# リリースビルドを実行します。
# 依存関係はすでにビルドされているため、このステップは主にアプリケーションコードのコンパイルとリンクになります。
RUN cargo build --release

# ---------------------------------------------------------------
# 実行ステージ
# ビルドされたバイナリと必要なファイルのみを含む軽量なイメージを作成します
# ---------------------------------------------------------------
FROM debian:12-slim

# タイムゾーンの設定 (任意)
RUN apt-get update && apt-get install -y tzdata --no-install-recommends && rm -rf /var/lib/apt/lists/*
ENV TZ=Asia/Tokyo
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# 作業ディレクトリ
WORKDIR /app

# ビルドステージからビルドされた実行ファイルをコピーします。
COPY --from=app_builder /app/target/release/todo-web ./todo-web

# アプリケーションが実行時に参照するファイルをコピーします。
# .envファイルと、マイグレーションファイル、静的ファイル、テンプレートファイルをコピーします。
COPY --from=app_builder /app/.env ./.env

# ビルド時に生成されたdatabase.dbは最終イメージに含めません。
# 実行時にはマウントされたボリュームからdatabase.dbが提供されることを想定します。
COPY --from=app_builder /app/migrations ./migrations
COPY --from=app_builder /app/static ./static
COPY --from=app_builder /app/templates ./templates

# app_builderステージで sqlx-cli のバイナリを一時的にビルドしてコピー
COPY --from=app_builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Entrypoint スクリプトを用意
RUN mkdir -p /docker-entrypoint.d
COPY ./shell/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

# entrypoint スクリプトを追加
COPY ./shell/exec_migration.sh /docker-entrypoint.d/exec_migration.sh
RUN chmod +x /docker-entrypoint.d/exec_migration.sh

# ポートの公開
EXPOSE 8080

# コンテナ起動時にWebアプリを実行します。
CMD ["./todo-web"]

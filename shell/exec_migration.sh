#!/bin/sh
set -e

# DBディレクトリ作成
mkdir -p db

# マイグレーション実行
sqlx migrate run


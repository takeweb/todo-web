#!/bin/sh
set -e

# /docker-entrypoint.d/ 内のスクリプトを実行
if [ -d /docker-entrypoint.d ]; then
  for f in /docker-entrypoint.d/*; do
    case "$f" in
      *.sh)
        echo "Running $f"
        . "$f"
        ;;
      *)
        echo "Ignoring $f"
        ;;
    esac
  done
fi

# 本体を起動
exec "$@"
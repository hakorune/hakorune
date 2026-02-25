#!/usr/bin/env bash
set -euo pipefail

# rename_nyash_to_hako.sh — 拡張子/参照の一括変換（安全志向）
#
# 目的:
#  1) 拡張子 .nyash → .hako にファイルをリネーム（git mv）
#  2) テキスト中の ".nyash" リテラルを ".hako" に書換
#
# 既定は DRY-RUN（変更無し）。
# 実行例:
#  DRY_RUN=0 RENAME_EXT=1 REWRITE_TEXT=1 ./tools/dev/rename_nyash_to_hako.sh
#
# 環境変数:
#  - DRY_RUN=1: 乾行（既定1）
#  - RENAME_EXT=1: .nyash → .hako のファイルリネームを有効化
#  - REWRITE_TEXT=1: テキスト置換（.nyash→.hako）を有効化
#  - SKIP_GLOBS: 追加スキップ（コロン区切り、例: "dist:build:some_dir"）

DRY_RUN="${DRY_RUN:-1}"
RENAME_EXT="${RENAME_EXT:-0}"
REWRITE_TEXT="${REWRITE_TEXT:-0}"

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

log() { echo "[rename] $*" >&2; }

should_skip() {
  case "$1" in
    ./target/*|.git/*|./.git/*|./.github/*|./node_modules/*|./target) return 0 ;;
  esac
  if [ -n "${SKIP_GLOBS:-}" ]; then
    IFS=':' read -r -a gl <<<"$SKIP_GLOBS"
    local f="$1"
    for g in "${gl[@]}"; do
      case "$f" in
        ./$g/*|./$g) return 0 ;;
      esac
    done
  fi
  return 1
}

# 1) ファイル拡張子のリネーム（git mv）
if [ "$RENAME_EXT" = "1" ]; then
  log "Scanning files to rename (.nyash → .hako)"
  mapfile -t files < <(git ls-files | grep -E '\.nyash$' || true)
  for f in "${files[@]}"; do
    [ -z "$f" ] && continue
    if should_skip "./$f"; then continue; fi
    dest="${f%.nyash}.hako"
    if [ "$DRY_RUN" = "1" ]; then
      log "DRY-RUN git mv '$f' '$dest'"
    else
      log "git mv '$f' '$dest'"
      git mv "$f" "$dest" || { log "WARN: git mv failed: $f"; }
    fi
  done
fi

# 2) テキスト置換（.nyash → .hako）
if [ "$REWRITE_TEXT" = "1" ]; then
  log "Rewriting text occurrences (.nyash → .hako)"
  # 候補ファイル（テキストファイル）: git 管理対象のみ、バイナリ除外
  mapfile -t tfs < <(git ls-files | xargs file | rg ':\s+text' -n | cut -d: -f1)
  for f in "${tfs[@]}"; do
    [ -z "$f" ] && continue
    if should_skip "./$f"; then continue; fi
    if rg -q '\.nyash\b' "$f"; then
      if [ "$DRY_RUN" = "1" ]; then
        log "DRY-RUN sed -i '.nyash'→'.hako' in $f"
      else
        tmp="$(mktemp)"
        sed 's/\.nyash\b/.hako/g' "$f" > "$tmp" && mv "$tmp" "$f"
      fi
    fi
  done
fi

log "Done (DRY_RUN=$DRY_RUN RENAME_EXT=$RENAME_EXT REWRITE_TEXT=$REWRITE_TEXT)"
exit 0


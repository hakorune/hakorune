#!/usr/bin/env bash
set -euo pipefail

TAG="sync-wasm-lane"
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
PHASE29CC_README="$ROOT_DIR/docs/development/current/main/phases/phase-29cc/README.md"
NOW_DOC="$ROOT_DIR/docs/development/current/main/10-Now.md"

LOCK_DOC=""
NEXT_ID=""
NEXT_TASK_TEXT=""
README_NOTE=""

usage() {
  cat <<'USAGE'
Usage:
  bash tools/selfhost/sync_wasm_lane_state.sh \
    --lock-doc docs/development/current/main/phases/phase-29cc/29cc-163-...-ssot.md \
    --next-id WSM-P5-min5 \
    --next-task-text ".hako emitter/binary writer 実体路を 1 shape 接続し、bridge fallback 非依存 case を lock する。" \
    --readme-note "1-shape real `.hako` emitter/binary writer route lock"

Sync targets:
  - CURRENT_TASK.md (Immediate Next #1 line)
  - docs/development/current/main/phases/phase-29cc/README.md (wasm lane active next line)
  - docs/development/current/main/10-Now.md (append lock doc line if missing)
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --lock-doc)
      LOCK_DOC="${2:-}"
      shift 2
      ;;
    --next-id)
      NEXT_ID="${2:-}"
      shift 2
      ;;
    --next-task-text)
      NEXT_TASK_TEXT="${2:-}"
      shift 2
      ;;
    --readme-note)
      README_NOTE="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[$TAG] unknown arg: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$LOCK_DOC" || -z "$NEXT_ID" || -z "$NEXT_TASK_TEXT" || -z "$README_NOTE" ]]; then
  echo "[$TAG] required args are missing" >&2
  usage >&2
  exit 2
fi

for f in "$CURRENT_TASK_DOC" "$PHASE29CC_README" "$NOW_DOC" "$ROOT_DIR/$LOCK_DOC"; do
  if [[ ! -f "$f" ]]; then
    echo "[$TAG] missing file: $f" >&2
    exit 1
  fi
done

SYNC_NEXT_ID="$NEXT_ID" SYNC_NEXT_TASK_TEXT="$NEXT_TASK_TEXT" perl -0777 -i -pe '
  my $id = $ENV{SYNC_NEXT_ID};
  my $text = $ENV{SYNC_NEXT_TASK_TEXT};
  s/^1\. `WSM-P5-min[0-9]+`（\.hako-only roadmap P5）: .*$/1. `$id`（.hako-only roadmap P5）: $text/m
    or die "[sync-wasm-lane] failed to update CURRENT_TASK immediate-next line\n";
' "$CURRENT_TASK_DOC"

SYNC_NEXT_ID="$NEXT_ID" SYNC_README_NOTE="$README_NOTE" perl -0777 -i -pe '
  my $id = $ENV{SYNC_NEXT_ID};
  my $note = $ENV{SYNC_README_NOTE};
  s/^  - wasm lane active next: `WSM-P5-min[0-9]+`（.*）$/  - wasm lane active next: `$id`（$note）/m
    or die "[sync-wasm-lane] failed to update phase-29cc README active-next line\n";
' "$PHASE29CC_README"

if ! grep -qF "\`$LOCK_DOC\`" "$NOW_DOC"; then
  last_entry=$(grep -nE '^[0-9]+\. `docs/development/current/main/phases/phase-29cc/29cc-[^`]+`$' "$NOW_DOC" | tail -n1 || true)
  if [[ -z "$last_entry" ]]; then
    echo "[$TAG] failed: no phase-29cc lock list entries found in 10-Now" >&2
    exit 1
  fi
  last_line="${last_entry%%:*}"
  last_text="${last_entry#*:}"
  last_num="$(echo "$last_text" | sed -E 's/^([0-9]+)\..*$/\1/')"
  new_num=$((last_num + 1))
  new_line="${new_num}. \`${LOCK_DOC}\`"
  tmp="$(mktemp)"
  awk -v n="$last_line" -v line="$new_line" '{
    print
    if (NR == n) print line
  }' "$NOW_DOC" > "$tmp"
  mv "$tmp" "$NOW_DOC"
fi

echo "[$TAG] synced wasm lane pointers"

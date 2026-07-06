#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-mir-json-cache-wrapper-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

HAKO_BIN="$ROOT_DIR/tools/bin/hako"
APP="$ROOT_DIR/examples/jit_demo.hako"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"

guard_require_files "$TAG" "$HAKO_BIN" "$APP" "$ENV_DOC"
guard_require_command "$TAG" find

TMP_DIR="$(mktemp -d /tmp/hako-mir-json-cache-wrapper.XXXXXX)"
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

CACHE_DIR="$TMP_DIR/cache"
OUT1="$TMP_DIR/one.mir.json"
OUT2="$TMP_DIR/two.mir.json"
LOG1="$TMP_DIR/one.log"
LOG2="$TMP_DIR/two.log"

HAKO_MIR_JSON_CACHE_DIR="$CACHE_DIR" \
  bash "$HAKO_BIN" --backend mir --emit-mir-json "$OUT1" "$APP" >"$LOG1" 2>&1 || {
    tail -n 120 "$LOG1" || true
    guard_fail "$TAG" "first emit failed"
  }

first_count="$(find "$CACHE_DIR" -type f | wc -l | tr -d ' ')"
if [ "$first_count" != "1" ]; then
  guard_fail "$TAG" "expected one cache file after first emit, got $first_count"
fi

HAKO_MIR_JSON_CACHE_DIR="$CACHE_DIR" \
  bash "$HAKO_BIN" --backend mir --emit-mir-json "$OUT2" "$APP" >"$LOG2" 2>&1 || {
    tail -n 120 "$LOG2" || true
    guard_fail "$TAG" "second emit failed"
  }

second_count="$(find "$CACHE_DIR" -type f | wc -l | tr -d ' ')"
if [ "$second_count" != "1" ]; then
  guard_fail "$TAG" "cache key drifted across equivalent output paths"
fi

if ! cmp -s "$OUT1" "$OUT2"; then
  guard_fail "$TAG" "cache hit output differs from first emit"
fi

if ! grep -q 'HAKO_MIR_JSON_CACHE' "$ENV_DOC"; then
  guard_fail "$TAG" "cache env knobs missing from environment reference"
fi

cat <<'REPORT'
output_contract=hako-mir-json-cache-wrapper-guard-v0
cache_status=miss_then_hit
equivalent_output_paths_share_key=1
outputs_equal=1
env_reference_documented=1
summary=ok
REPORT

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_python_template_c_retirement.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

expect_retired_without_flag() {
  local name="$1"
  shift
  local out="$TMP_DIR/$name.out"
  local err="$TMP_DIR/$name.err"
  set +e
  "$@" >"$out" 2>"$err"
  local rc=$?
  set -e
  test "$rc" -ne 0
  grep -q 'Python-template C replacement front is retired from normal runs' "$err"
}

expect_allows_explicit_baseline() {
  local name="$1"
  shift
  local out="$TMP_DIR/$name.out"
  local err="$TMP_DIR/$name.err"
  set +e
  "$@" >"$out" 2>"$err"
  local rc=$?
  set -e
  test "$rc" -ne 0
  grep -q -- '--mimalloc-library PATH or --allow-ldconfig-discovery is required' "$err"
}

expect_retired_without_flag \
  hakozuna_without_flag \
  python3 "$ROOT/tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py" \
    --out-dir "$TMP_DIR/hakozuna" \
    --replacement-front-native-slot-mode

expect_allows_explicit_baseline \
  hakozuna_with_flag \
  python3 "$ROOT/tools/allocator/hakozuna_mixed_ws_ldpreload_compare.py" \
    --out-dir "$TMP_DIR/hakozuna" \
    --allow-python-template-c-bridge-baseline \
    --replacement-front-native-slot-mode

expect_retired_without_flag \
  hakmem_without_flag \
  python3 "$ROOT/tools/allocator/hakmem_fixture_ldpreload_compare.py" \
    --fixture tiny-hot-system \
    --out-dir "$TMP_DIR/hakmem" \
    --replacement-front-native-slot-mode

expect_allows_explicit_baseline \
  hakmem_with_flag \
  python3 "$ROOT/tools/allocator/hakmem_fixture_ldpreload_compare.py" \
    --fixture tiny-hot-system \
    --out-dir "$TMP_DIR/hakmem" \
    --allow-python-template-c-bridge-baseline \
    --replacement-front-native-slot-mode

echo "[TEST/OK] python_template_c_bridge_retirement"
